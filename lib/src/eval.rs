use std::collections::VecDeque;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;

use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::scan;
use crate::FormattedValue;
use crate::Step;
use crate::Token;
use crate::{Error, Expr};

#[derive(Debug, Serialize)]
pub struct Evaluation {
    pub command: Option<Vec<Token>>,
    pub steps: Vec<Step>,
    pub result: Option<i64>,
    pub token: Option<Token>,
    pub error: Option<String>,
}

pub fn evaluate(command: &str) -> Evaluation {
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
pub fn evaluatetojson(command: &str) -> String {
    let eval = evaluate(command);

    match serde_json::to_string(&eval) {
        Ok(json) => json,
        Err(e) => format!(
            "{{\"command\":\"{}\",\"error\":\"error: {}\",\"steps\":[]}}",
            command,
            e.to_string()
        ),
    }
}

fn evaluate_exprs(
    expressions: &[Expr],
    steps: &mut Vec<Step>,
    mut tag_counter: &mut usize,
) -> Result<Expr, Error> {
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
            ("~", i64::not),
            ("-", i64::neg),
            ("!", |v| if v == 0 { 1 } else { 0 }),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("*", i64::mul), ("/", i64::div), ("%", i64::rem_euclid)],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("+", i64::add), ("-", i64::sub)],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[
            (">", |a, b| if a > b { 1 } else { 0 }),
            ("<", |a, b| if a < b { 1 } else { 0 }),
            (">=", |a, b| if a >= b { 1 } else { 0 }),
            ("<=", |a, b| if a <= b { 1 } else { 0 }),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[
            ("==", |a, b| if a == b { 1 } else { 0 }),
            ("!=", |a, b| if a != b { 1 } else { 0 }),
        ],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("<<", i64::shl), (">>", i64::shr)],
    );

    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("&", i64::bitand)]);
    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("^", i64::bitxor)]);
    exprs = evaluate_binary_op(exprs, steps, &mut tag_counter, &[("|", i64::bitor)]);

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("&&", |a, b| if (a != 0) & (b != 0) { 1 } else { 0 })],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("^^", |a, b| if (a != 0) ^ (b != 0) { 1 } else { 0 })],
    );

    exprs = evaluate_binary_op(
        exprs,
        steps,
        &mut tag_counter,
        &[("||", |a, b| if (a != 0) | (b != 0) { 1 } else { 0 })],
    );

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

fn evaluate_binary_op(
    mut exprs: VecDeque<Expr>,
    steps: &mut Vec<Step>,
    tag_counter: &mut usize,
    op_table: &[(&'static str, fn(i64, i64) -> i64)],
) -> VecDeque<Expr> {
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
                                format: Some(FormattedValue::from_i64(result_num)),
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

fn evaluate_unary_op(
    mut exprs: VecDeque<Expr>,
    steps: &mut Vec<Step>,
    tag_counter: &mut usize,
    op_table: &[(&'static str, fn(i64) -> i64)],
) -> VecDeque<Expr> {
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
                                let neg_operand_num = -operand_num;
                                let neg_operand_tok = Token {
                                    text: format!("{}", neg_operand_num),
                                    tag: operand_tok.tag,
                                    format: Some(FormattedValue::from_i64(neg_operand_num)),
                                };

                                exprs.push_back(Expr::NumberToken(neg_operand_num, neg_operand_tok));
                            } else {

                                let result_num = op_fn(operand_num);
                                let result_tok = Token {
                                    text: result_num.to_string(),
                                    tag: Some(*tag_counter),
                                    format: Some(FormattedValue::from_i64(result_num)),
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
