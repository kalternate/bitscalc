use std::fmt::Display;

use serde::Serialize;

use crate::Token;

#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub op: String,
    pub left: Option<Token>,
    pub right: Token,
    pub result: Token,
}

impl Step {
    pub fn unary(op: impl ToString, right: Token, result: Token) -> Self {
        Step {
            op: op.to_string(),
            left: None,
            right,
            result,
        }
    }

    pub fn binary(op: impl ToString, left: Token, right: Token, result: Token) -> Self {
        Step {
            op: op.to_string(),
            left: Some(left),
            right,
            result,
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.left {
            Some(left) => write!(
                f,
                "{} {} {} = {}",
                left.text, self.op, self.right.text, self.result.text
            ),
            None => write!(f, "{}{} = {}", self.op, self.right.text, self.result.text),
        }
    }
}
