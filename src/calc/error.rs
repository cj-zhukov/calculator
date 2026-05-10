use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CalcError {
    #[error("invalid token: {0}")]
    BadToken(char),

    #[error("mismatched parentheses")]
    MismatchedParens,

    #[error("not enough operands")]
    NotEnoughOperands,

    #[error("invalid expression")]
    InvalidExpression,

    #[error("division by zero")]
    DivisionByZero,
}
