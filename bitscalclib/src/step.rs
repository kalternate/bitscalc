use std::fmt::Display;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::FormattedValue;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct  Step {
    pub op: String,
    pub left: Option<FormattedValue>,
    pub right: FormattedValue,
    pub result: FormattedValue
}

impl Step {
    pub fn unary(op: impl ToString, right: i64, result: i64) -> Self {
        Step {
            op: op.to_string(),
            left: None,
            right: FormattedValue::from_i64(right),
            result: FormattedValue::from_i64(result)
        }
    }

    pub fn binary(op: impl ToString, left: i64, right: i64, result: i64) -> Self {
        Step {
            op: op.to_string(),
            left: Some(FormattedValue::from_i64(left)),
            right: FormattedValue::from_i64(right),
            result: FormattedValue::from_i64(result)
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.left {
            Some(left) => write!(f, "{} {} {} = {}", left.dec, self.op, self.right.dec, self.result.dec),
            None => write!(f, "{}{} = {}", self.op, self.right.dec, self.result.dec),
        }
    }
}