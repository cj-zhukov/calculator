use calculator::{Calc, Error};

fn main() -> Result<(), Error> {

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = Calc::parse(input)?;
                let expr = Calc::expression(tokens);
                let res = Calc::avaluate(expr).unwrap();
                println!("{}", res);
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
