#[derive(Debug, PartialEq)]
pub enum CalcError {
    BadToken(char),
    MismatchedParens,
    NotEnoughOperands,
    DivisionByZero,
}
