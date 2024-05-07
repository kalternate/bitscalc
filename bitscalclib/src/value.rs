use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct FormattedValue {
    pub dec: String,
    pub hex: String,
    pub bin: String
}

impl FormattedValue {
    pub fn from_i64(value: i64) -> Self {
        FormattedValue {
            dec: format!("{}", value),
            hex: format!("{:#x}", value),
            bin: format!("{:#b}", value),
        }
    }
}




