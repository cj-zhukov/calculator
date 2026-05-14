pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod token;

use error::CalcError;

pub fn calculate(expr: &str) -> Result<f32, CalcError> {
    let tokens = lexer::tokenize(expr)?;
    let postfix = parser::to_postfix(tokens);
    eval::eval_postfix(postfix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() -> Result<(), CalcError> {
        let res = calculate("(5 + 5) * 2 / 2")?;
        assert_eq!(res, 10.0);
        Ok(())
    }
}
