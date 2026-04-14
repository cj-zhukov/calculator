use std::io::{self, Write};

use thiserror::Error;

use crate::calc::{error::CalcError, evaluate};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Calc(#[from] CalcError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn run() -> Result<(), AppError> {
    println!("calculator");

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!("no input provided");
            break;
        }

        let input = input.trim();

        // exit calculator
        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        match evaluate(input) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
