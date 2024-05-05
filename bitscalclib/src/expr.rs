use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Expr {
    Op(&'static str),
    ParenOpen,
    ParenClose,
    Number(i64)
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Op(op) => write!(f, "{}", op),
            Expr::ParenOpen => write!(f, "("),
            Expr::ParenClose => write!(f, ")"),
            Expr::Number(number) => write!(f, "{}", number),
        }
    }
}