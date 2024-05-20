use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FormattedValue {
    pub dec: String,
    pub hex: String,
    pub bin: String,
}

impl FormattedValue {
    pub fn from_i64(value: i64) -> Self {
        FormattedValue {
            dec: format!("{}", value),
            hex: format!("{:#x}", value),
            bin: make_bin_format(value),
        }
    }
}

fn make_bin_format(value: i64) -> String {
    let unspaced: Vec<char> = format!("{:b}", value).chars().collect();
    let mut spaced = String::new();

    if unspaced.len() % 4 != 0 {
        for _ in 0..4-(unspaced.len() % 4) {
            spaced.push('0')
        }
    }

    for i in 0..unspaced.len() {
        let l = unspaced.len()-i;
        if l % 4 == 0 {
            spaced.push(' ')
        }

        spaced.push(unspaced[i]);
    }

    spaced
}