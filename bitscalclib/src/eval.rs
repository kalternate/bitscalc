use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::ops::Neg;
use std::ops::Shr;
use std::ops::Shl;

use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::scan;
use crate::Step;
use crate::{Error, Expr};
use crate::FormattedValue;
#[derive(Debug, Serialize)]
pub struct Evaluation {
    pub command: String,
    pub steps: Vec<Step>,
    pub result: Option<i64>,
    pub format: Option<FormattedValue>,
    pub error: Option<String>
}

pub fn evaluate(command: &str) -> Evaluation {

    match scan(command) {
    Ok(exprs) => {
        let mut steps = Vec::new();
        let eval = evaluate_exprs(&exprs, &mut steps);
    
        let (result, format, error) = match eval {
            Ok(value) => {
                (Some(value), Some(FormattedValue::from_i64(value)), None)
            },
            Err(err) => {
                (None, None, Some(err.0))
            },
        };

        Evaluation {
            command: command.to_string(),
            steps,
            result,
            format,
            error
        }
    },
    Err(error) => Evaluation {
        command: command.to_string(),
        steps: Vec::new(),
        result: None,
        format: None,
        error: Some(error.0)
        },
    }
}

#[wasm_bindgen]
pub fn evaluatetojson(command: &str) -> String {
    let eval = evaluate(command);

    match serde_json::to_string(&eval) {
        Ok(json) =>json,
        Err(e) => format!("{{\"command\":\"{}\",\"error\":\"error: {}\",\"steps\":[]}}", command, e.to_string()),
    }
}


fn evaluate_exprs(expressions: &[Expr], steps: &mut Vec<Step>) -> Result<i64, Error> {

    let mut index = 0;
    let mut exprs = VecDeque::new();

    if expressions.is_empty() {
        return Err(Error(format!("parse error: cannot fully evaluate expression")));
    }

    while let Some(expr) = expressions.get(index) {
        if let Expr::ParenOpen = expr {
            let start = index;
            let mut end = index + 1;
            let mut run = true;
            let mut depth = 0;

            while run {
                match expressions.get(end) {
                    Some(ex) => {
                        match ex {
                            Expr::ParenOpen => {depth += 1;},
                            Expr::ParenClose => {
                                if depth > 0 {
                                    depth -= 1;
                                } else {
                                    run = false;
                                }
                            },
                            _ => {}
                            
                        }
                    },
                    None => todo!(),
                }
                end += 1;
            }

            let evaled = evaluate_exprs(expressions.get(start+1..end-1).unwrap(), steps)?;
            exprs.push_back(Expr::Number(evaled));
            
            index = end;
        } else {
            exprs.push_back(expr.clone());
            index += 1;
        }
    }

    exprs = evaluate_unary_op(exprs, steps,&[("~", i64::not), ("-", i64::neg), ("!", |v| if v == 0 { 1 } else { 0 })]);
    exprs = evaluate_binary_op(exprs, steps, &[("*", i64::mul), ("/", i64::div), ("%", i64::rem_euclid)]);
    exprs = evaluate_binary_op(exprs, steps, &[("+", i64::add), ("-", i64::sub)]);
    exprs = evaluate_binary_op(exprs, steps, &[("<<", i64::shl), (">>", i64::shr)]);

    exprs = evaluate_binary_op(exprs, steps, &[("&", i64::bitand)]);
    exprs = evaluate_binary_op(exprs, steps, &[("^", i64::bitxor)]);
    exprs = evaluate_binary_op(exprs, steps, &[("|", i64::bitor)]);

    if exprs.len() != 1 {
        Err(Error(format!("parse error: cannot fully evaluate expression")))
    } else {
        if let Some(Expr::Number(result)) = exprs.pop_front() {
            Ok(result)
        } else {
            Err(Error(format!("parse error: cannot fully evaluate expression")))
        }
    }
}

fn evaluate_binary_op(mut exprs: VecDeque<Expr>, steps: &mut Vec<Step>, op_table: &[(&'static str, fn(i64, i64) -> i64)]) -> VecDeque<Expr> {
    let mut results = VecDeque::new();
    
    while let Some(expr) = exprs.pop_front() {
        let mut keep_expr = true;
        if let Expr::Number(left) = expr {
            if let Some(Expr::Op(cur_symbol)) = exprs.get(0).cloned() {
                if let Some(Expr::Number(right)) = exprs.get(1).cloned() {
                    for (symbol, op_fn) in op_table {
                        if cur_symbol == *symbol {
                            exprs.pop_front();
                            exprs.pop_front();
                            let result = op_fn(left, right);
                            exprs.push_front(Expr::Number(result));
                            steps.push(Step::binary(symbol, left, right, result));
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
    };

    results
}

fn evaluate_unary_op(mut exprs: VecDeque<Expr>, steps: &mut Vec<Step>, op_table: &[(&'static str, fn(i64) -> i64)]) -> VecDeque<Expr> {
    let mut results = VecDeque::new();
    
    while let Some(expr) = exprs.pop_back() {
        let mut keep_expr = true;

        if let Expr::Number(operand) = expr {
            if !exprs.is_empty() {
                if let Some(Expr::Op(cur_symbol)) = exprs.get(exprs.len()-1).cloned() {
                    for (symbol, op_fn) in op_table {
                        if cur_symbol == *symbol {

                            if cur_symbol == "-" && exprs.len() > 1 {
                                if let Some(Expr::Number(_)) = exprs.get(exprs.len()-2) {
                                    break;
                                }
                            }

                            exprs.pop_back();
                            let result = op_fn(operand);
                            exprs.push_back(Expr::Number(result));

                            if cur_symbol != "-" {
                                steps.push(Step::unary(symbol, operand, result));
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
    };

    results
}