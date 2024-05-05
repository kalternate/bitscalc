use std::fmt::Display;

pub struct  Step {
    op: &'static str,
    left: Option<i64>,
    right: i64,
    result: i64
}

impl Step {
    pub fn unary(op: &'static str, right: i64, result: i64) -> Self {
        Step {
            op,
            left: None,
            right,
            result
        }
    }

    pub fn binary(op: &'static str, left: i64, right: i64, result: i64) -> Self {
        Step {
            op,
            left: Some(left),
            right,
            result
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.left {
            Some(left) => write!(f, "{} {} {} = {}", left, self.op, self.right, self.result),
            None => write!(f, "{}{} = {}", self.op, self.right, self.result),
        }
    }
}