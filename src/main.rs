use std::io::{self, Write};

use calculator::{Calc, Error};

fn main() -> Result<(), Error> {
    println!("calculator");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match Calc::parse(input) {
            Ok(tokens) => {
                let expr = Calc::expression(tokens);
                match Calc::avaluate(expr) {
                    Some(res) => println!("{}", res),
                    None => println!("Error: evaluation failed"),
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(())
}
