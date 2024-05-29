use serde::Serialize;

use crate::{Expr, FormattedValue, Value};

#[derive(Debug, Serialize, Clone)]
pub struct Token {
    pub text: String,
    pub tag: Option<usize>,
    pub format: Option<FormattedValue>,
}

impl<V: Value> From<Expr<V>> for Token {
    fn from(expr: Expr<V>) -> Self {
        match expr {
            Expr::Op(op) => Token {
                text: op.into(),
                tag: None,
                format: None,
            },
            Expr::ParenOpen => Token {
                text: "(".into(),
                tag: None,
                format: None,
            },
            Expr::ParenClose => Token {
                text: ")".into(),
                tag: None,
                format: None,
            },
            Expr::NumberToken(_, token) => token,
        }
    }
}
