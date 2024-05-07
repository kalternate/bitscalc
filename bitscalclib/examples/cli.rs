use bitscalclib::evaluate;
use console::Term;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    loop {

        let mut terminal = Term::stdout();

        write!(terminal, "bitscalc$ ")?;

        while let Ok(input) = terminal.read_line() {
           
            let eval = evaluate(&input);

            if let Some(result) = eval.format {
                for step in eval.steps {
                    if let Some(left) = step.left {
                        println!("{} {} {} = {}", left.dec, step.op, step.right.dec, step.result.dec)
                    } else {
                        println!("{}{} = {}", step.op, step.right.dec, step.result.dec)

                    }
                }
                println!("{} ", result.dec)
            }

            write!(terminal, "bitscalc$ ")?;
        }
    }
}