#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(Bracket),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bracket {
    Open,
    Close,
}
