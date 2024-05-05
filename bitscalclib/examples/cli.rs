use bitscalclib::{evaluate_steps, tokenize};
use console::Term;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    loop {

        let mut terminal = Term::stdout();

        write!(terminal, "bitscalc$ ")?;

        while let Ok(input) = terminal.read_line() {
            let tokens = tokenize(&input);

            if !tokens.is_empty() {
                match evaluate_steps(&tokens) {
                    Ok((value, steps)) => {
                        for step in steps {
                            writeln!(terminal, "{}", step)?;
                        }
                        writeln!(terminal, "> {}", value)?;
                    },
                    Err(err) => writeln!(terminal, "{}", err.0)?,
                };
            }


            write!(terminal, "bitscalc$ ")?;
        }
    }
}