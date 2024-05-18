use std::fmt::Display;

use crate::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Op(&'static str),
    ParenOpen,
    ParenClose,
    NumberToken(i64, Token),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Op(op) => write!(f, "{}", op),
            Expr::ParenOpen => write!(f, "("),
            Expr::ParenClose => write!(f, ")"),
            Expr::NumberToken(_, token) => write!(f, "{}", token.text),
        }
    }
}
