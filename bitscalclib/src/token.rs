use crate::Expr;


pub fn tokenize(s: &str) -> Vec<Expr> {
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut tokens = Vec::new();

    while let Some(start_char) = chars.get(start) {
        if start_char.is_numeric() {
            let mut end = start + 1;
            while chars.get(end).cloned().is_some_and(char::is_numeric) {
                end += 1;
            }
            let token_str: String = chars.get(start..end).unwrap().iter().collect();
            tokens.push(Expr::Number(token_str.parse().unwrap()));
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


    tokens
}

