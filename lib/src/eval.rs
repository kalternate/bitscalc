use std::collections::VecDeque;

use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::scan;
use crate::FormattedValue;
use crate::Step;
use crate::Token;
use crate::Value;
use crate::{Error, Expr};

#[derive(Debug, Serialize)]
pub struct Evaluation<V> {
    pub command: Option<Vec<Token>>,
    pub steps: Vec<Step>,
    pub result: Option<V>,
    pub token: Option<Token>,
    pub error: Option<String>,
}

pub fn evaluate<V: Value>(command: &str) -> Evaluation<V> {
    macro_rules! eval_error {
        ($err:expr) => {
            Evaluation {
                command: None,
                steps: Vec::new(),
                result: None,
                token: None,
                error: Some($err.into()),
            }
        };
    }

    let mut tag_counter = 1;

    match scan(command, &mut tag_counter) {
        Ok(exprs) => {
            let mut steps = Vec::new();
            match evaluate_exprs(&exprs, &mut steps, &mut tag_counter) {
                Ok(result_expr) => {
                    if let Expr::NumberToken(result_num, result_token) = result_expr {
                        let command =
                            Some(exprs.into_iter().map(|expr| Token::from(expr)).collect());

                        Evaluation {
                            command,
                            steps,
                            result: Some(result_num),
                            token: Some(result_token),
                            error: None,
                        }
                    } else {
                        eval_error!("error: expression didn't reduce to number")
                    }
                }
                Err(error) => eval_error!(error.0),
            }
        }
        Err(error) => eval_error!(error.0),
    }
}

#[wasm_bindgen]
pub fn evaluatetojson(command: &str, signed: bool, size: i64) -> String {

    macro_rules! handle_eval {
        ($eval:expr) => {
            match serde_json::to_string(&$eval) {
                Ok(json) => json,
                Err(e) => format!(
                    "{{\"command\":\"{}\",\"error\":\"error: {}\",\"steps\":[]}}",
                    command,
                    e.to_string()
                ),
            }
        };
    }

    match size {
        64 => {
            if signed {
                handle_eval!(evaluate::<i64>(command))
            } else {
                handle_eval!(evaluate::<u64>(command))
            }
        },

        32 => {
            if signed {
                handle_eval!(evaluate::<i32>(command))
            } else {
                handle_eval!(evaluate::<u32>(command))
            }
        },

        16 => {
            if signed {
                handle_eval!(evaluate::<i16>(command))
            } else {
                handle_eval!(evaluate::<u16>(command))
            }
        },

        8 => {
            if signed {
                handle_eval!(evaluate::<i8>(command))
            } else {
                handle_eval!(evaluate::<u8>(command))
            }
        },

        _ => {
            format!(
                "{{\"command\":\"{}\",\"error\":\"error: invalid size\",\"steps\":[]}}",
                command
            )
        }
    }
}

fn evaluate_exprs<V: Value>(
    expressions: &[Expr<V>],
    steps: &mut Vec<Step>,
    mut tag_counter: &mut usize,
) -> Result<Expr<V>, Error> {
    let mut index = 0;
    let mut exprs = VecDeque::new();

    if expressions.is_empty() {
        return Err(Error(format!(
            "parse error: cannot fully evaluate expression"
        )));
    }

    while let Some(expr) = expressions.get(index) {
        if let Expr::ParenOpen = expr {
            let start = index;
            let mut end = index + 1;
            let mut run = true;
            let mut depth = 0;

            while run {
                match expressions.get(end) {
                    Some(ex) => match ex {
                        Expr::ParenOpen => {
                            depth += 1;
                        }
                        Expr::ParenClose => {
                            if depth > 0 {
                                depth -= 1;
                            } else {
                                run = false;
                            }
                        }
                        _ => {}
                    },
                    None => todo!(),
                }
                end += 1;
            }

            let evaled = evaluate_exprs(
                expressions.get(start + 1..end - 1).unwrap(),
                steps,
                tag_counter,
            )?;
            exprs.push_back(evaled);

            index = end;
        } else {
            exprs.push_back(expr.clone());
            index += 1;
        }
    }

    exprs = evaluate_unary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[
            ("~", V::bitwise_not),
            ("-", V::negative),
            ("!", V::logical_not),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[
            ("*", V::multiplication),
            ("/", V::division),
            ("%", V::remainder),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("+", V::addition), ("-", V::subtraction)],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("<<", V::left_bitshift), (">>", V::right_bitshift)],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[
            (">", V::greater_than),
            ("<", V::less_than),
            (">=", V::greater_than_or_equal),
            ("<=", V::less_than_or_equal),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("==", V::equals), ("!=", V::not_equals)],
    );

    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("&", V::bitwise_and)]);
    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("^", V::bitwise_xor)]);
    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("|", V::bitwise_or)]);

    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("&&", V::logical_and)]);

    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("^^", V::logical_xor)]);

    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("||", V::logical_or)]);

    if exprs.len() != 1 {
        Err(Error(format!(
            "parse error: cannot fully evaluate expression"
        )))
    } else {
        if let Some(expr) = exprs.pop_front() {
            Ok(expr)
        } else {
            Err(Error(format!(
                "parse error: cannot fully evaluate expression"
            )))
        }
    }
}

fn evaluate_binary_op<V: Value>(
    mut exprs: VecDeque<Expr<V>>,
    steps: &mut Vec<Step>,
    tag_counter: &mut usize,
    op_table: &[(&'static str, fn(V, V) -> V)],
) -> VecDeque<Expr<V>> {
    let mut results = VecDeque::new();

    while let Some(expr) = exprs.pop_front() {
        let mut keep_expr = true;
        if let Expr::NumberToken(left_num, left_tok) = expr.clone() {
            if let Some(Expr::Op(cur_symbol)) = exprs.get(0).cloned() {
                if let Some(Expr::NumberToken(right_num, right_tok)) = exprs.get(1).cloned() {
                    for (symbol, op_fn) in op_table {
                        if cur_symbol == *symbol {
                            exprs.pop_front();
                            exprs.pop_front();

                            let result_num = op_fn(left_num, right_num);

                            let result_tok = Token {
                                text: result_num.to_string(),
                                tag: Some(*tag_counter),
                                format: Some(FormattedValue::from_value(result_num)),
                            };

                            *tag_counter += 1;

                            exprs.push_front(Expr::NumberToken(result_num, result_tok.clone()));
                            steps.push(Step::binary(symbol, left_tok, right_tok, result_tok));
                            keep_expr = false;
                            break;
                        }
                    }
                }
            }
        }

        if keep_expr {
            results.push_back(expr);
        }
    }

    results
}

fn evaluate_unary_op<V: Value>(
    mut exprs: VecDeque<Expr<V>>,
    steps: &mut Vec<Step>,
    tag_counter: &mut usize,
    op_table: &[(&'static str, fn(V) -> V)],
) -> VecDeque<Expr<V>> {
    let mut results = VecDeque::new();

    while let Some(expr) = exprs.pop_back() {
        let mut keep_expr = true;

        if let Expr::NumberToken(operand_num, operand_tok) = expr.clone() {
            if !exprs.is_empty() {
                if let Some(Expr::Op(cur_symbol)) = exprs.get(exprs.len() - 1).cloned() {
                    for (symbol, op_fn) in op_table {
                        if cur_symbol == *symbol {
                            if cur_symbol == "-" && exprs.len() > 1 {
                                if let Some(Expr::NumberToken(_, _)) = exprs.get(exprs.len() - 2) {
                                    break;
                                }
                            }

                            exprs.pop_back();

                            if cur_symbol == "-" {
                                let neg_operand_num = operand_num.negative();
                                let neg_operand_tok = Token {
                                    text: format!("{}", neg_operand_num),
                                    tag: operand_tok.tag,
                                    format: Some(FormattedValue::from_value(neg_operand_num)),
                                };

                                exprs
                                    .push_back(Expr::NumberToken(neg_operand_num, neg_operand_tok));
                            } else {
                                let result_num = op_fn(operand_num);
                                let result_tok = Token {
                                    text: result_num.to_string(),
                                    tag: Some(*tag_counter),
                                    format: Some(FormattedValue::from_value(result_num)),
                                };

                                *tag_counter += 1;

                                exprs.push_back(Expr::NumberToken(result_num, result_tok.clone()));

                                steps.push(Step::unary(symbol, operand_tok, result_tok));
                            }

                            keep_expr = false;
                            break;
                        }
                    }
                }
            }
        }

        if keep_expr {
            results.push_front(expr);
        }
    }

    results
}
