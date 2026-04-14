#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    Number(f32),
    Op(Operator),
    Bracket(Bracket),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bracket {
    Open,
    Close,
}
