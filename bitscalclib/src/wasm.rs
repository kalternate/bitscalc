use wasm_bindgen::prelude::wasm_bindgen;

use crate::{evaluate, tokenize};


#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct Evaluation {
    pub command: String,
    pub result: Option<Value>,
    pub error: Option<String>
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Value {
    pub dec: String,
    pub hex: String,
    pub bin: String
}

impl Value {
    pub fn from_i64(value: i64) -> Self {
        Value {
            dec: format!("{}", value),
            hex: format!("{:#x}", value),
            bin: format!("{:#b}", value),
        }
    }
}

#[wasm_bindgen]
pub fn evaluate_command(command: String) -> Evaluation {

    match tokenize(&command) {
    Ok(exprs) => {
        let eval_result = evaluate(&exprs);
    
        let mut error = None;

        let result = match eval_result {
            Ok(value) => {
                Some(Value::from_i64(value))
            },
            Err(err) => {
                error = Some(err.0);
                None
            },
        };

        Evaluation {
            command,
            result,
            error
        }
    },
    Err(error) => Evaluation {
        command,
        result: None,
        error: Some(error.0)
        },
    }
}


