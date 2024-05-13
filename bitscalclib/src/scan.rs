use crate::{Error, Expr, FormattedValue, Token};


pub fn scan(cmd: &str, tag_counter: &mut usize) -> Result<Vec<Expr>, Error> {
    let chars: Vec<char> = cmd.chars().collect();
    let mut start = 0;
    let mut tokens = Vec::new();

    while let Some(start_char) = chars.get(start) {
        if start_char.is_alphanumeric() {
            let mut end = start + 1;
            while chars.get(end).cloned().is_some_and(char::is_alphanumeric) {
                end += 1;
            }
            let token_str: String = chars.get(start..end).unwrap().iter().collect();
            
            let number = parse_numeric(&token_str)?;
            let token = Token{
                text: token_str,
                tag: Some(*tag_counter),
                format: Some(FormattedValue::from_i64(number)),
            };

            *tag_counter += 1;

            tokens.push(Expr::NumberToken(number, token));
            start = end;
        } else {
            match start_char {
                '+' => { tokens.push(Expr::Op("+")) }
                '-' => { tokens.push(Expr::Op("-")) }
                '*' => { tokens.push(Expr::Op("*")) }
                '/' => { tokens.push(Expr::Op("/")) }
                '%' => { tokens.push(Expr::Op("%")) }
                '|' => { tokens.push(Expr::Op("|")) }
                '&' => { tokens.push(Expr::Op("&")) }
                '^' => { tokens.push(Expr::Op("^")) }
                '!' => { tokens.push(Expr::Op("!")) }
                '~' => { tokens.push(Expr::Op("~")) }
                '(' => { tokens.push(Expr::ParenOpen) }
                ')' => { tokens.push(Expr::ParenClose) }

                '>' => {
                    if let Some(next) = chars.get(start+1) {
                        if *next == '>' {
                            start += 1;
                            tokens.push(Expr::Op(">>"))
                        }
                    }
                }

                '<' => {
                    if let Some(next) = chars.get(start+1) {
                        if *next == '<' {
                            start += 1;
                            tokens.push(Expr::Op("<<"))
                        }
                    }
                }

                _ => {  }
            };
            start += 1;
        }
    }


    Ok(tokens)
}

fn parse_numeric(token: &str) -> Result<i64, Error> {
    if token.len() < 3 {
        i64::from_str_radix(token, 10)
    } else {
        let (prefix, suffix) = token.split_at(2);
        if prefix == "0x" {
            i64::from_str_radix(suffix, 16)
        } else if prefix == "0b" {
            i64::from_str_radix(suffix, 2)
        } else {
            i64::from_str_radix(token, 10)
        }
    }.map_err(|_| Error(format!("error: cannot parse '{}'", token)))
}