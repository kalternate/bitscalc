use bitscalclib::evaluate;
use console::Term;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    loop {

        let mut terminal = Term::stdout();

        write!(terminal, "bitscalc$ ")?;

        while let Ok(input) = terminal.read_line() {
           
            let eval = evaluate(&input);

            if let Some(result) = eval.token.map(|t| t.format).flatten() {
                for step in eval.steps {
                    if let Some(left) = step.left {
                        println!("{} {} {} = {}", left.text, step.op, step.right.text, step.result.text)
                    } else {
                        println!("{}{} = {}", step.op, step.right.text, step.result.text)
                    }
                }
                println!("{} ", result.dec)
            }

            write!(terminal, "bitscalc$ ")?;
        }
    }
}