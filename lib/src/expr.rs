use std::fmt::Display;

use crate::Token;

#[derive(Debug, Clone)]
pub enum Expr<V> {
    Op(&'static str),
    ParenOpen,
    ParenClose,
    NumberToken(V, Token),
}

impl<V> Display for Expr<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Op(op) => write!(f, "{}", op),
            Expr::ParenOpen => write!(f, "("),
            Expr::ParenClose => write!(f, ")"),
            Expr::NumberToken(_, token) => write!(f, "{}", token.text),
        }
    }
}
