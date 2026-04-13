use super::error::CalcError;
use super::token::{Operator, Token};

pub fn eval_postfix(mut tokens: Vec<Token>) -> Result<f32, CalcError> {
    tokens.reverse();

    let mut stack: Vec<f32> = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            Token::Number(n) => stack.push(n as f32),

            Token::Op(op) => {
                let right = stack.pop().ok_or(CalcError::NotEnoughOperands)?;
                let left = stack.pop().ok_or(CalcError::NotEnoughOperands)?;

                let res = match op {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => {
                        if right == 0.0 {
                            return Err(CalcError::DivisionByZero);
                        }
                        left / right
                    }
                };

                stack.push(res);
            }

            _ => {}
        }
    }

    if stack.len() != 1 {
        return Err(CalcError::NotEnoughOperands);
    }

    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::token::{Operator, Token};

    fn n(x: u32) -> Token {
        Token::Number(x)
    }

    fn op(op: Operator) -> Token {
        Token::Op(op)
    }

    #[test]
    fn eval_add() {
        let input = vec![n(1), n(2), op(Operator::Add)];
        let res = eval_postfix(input).unwrap();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn eval_all_ops() {
        // (5 + 5) * 2 = 20 → postfix: 5 5 + 2 *
        let input = vec![n(5), n(5), op(Operator::Add), n(2), op(Operator::Mul)];
        let res = eval_postfix(input).unwrap();
        assert_eq!(res, 20.0);
    }

    #[test]
    fn eval_sub_order() {
        // 10 - 5 = 5 → postfix: 10 5 -
        let input = vec![n(10), n(5), op(Operator::Sub)];
        let res = eval_postfix(input).unwrap();
        assert_eq!(res, 5.0);
    }

    #[test]
    fn eval_div() {
        let input = vec![n(10), n(5), op(Operator::Div)];
        let res = eval_postfix(input).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn eval_div_by_zero() {
        let input = vec![n(10), n(0), op(Operator::Div)];
        let err = eval_postfix(input).unwrap_err();
        assert!(matches!(err, CalcError::DivisionByZero));
    }

    #[test]
    fn eval_not_enough_operands() {
        let input = vec![op(Operator::Add)];
        let err = eval_postfix(input).unwrap_err();
        assert!(matches!(err, CalcError::NotEnoughOperands));
    }

    #[test]
    fn eval_partial_expression() {
        // 1 +
        let input = vec![n(1), op(Operator::Add)];
        let err = eval_postfix(input).unwrap_err();
        assert!(matches!(err, CalcError::NotEnoughOperands));
    }

    #[test]
    fn eval_extra_operands_should_fail() {
        let input = vec![n(1), n(2), n(3), op(Operator::Add)];
        let err = eval_postfix(input).unwrap_err();
        assert!(matches!(err, CalcError::NotEnoughOperands));
    }

    #[test]
    fn eval_empty() {
        let input = vec![];
        let err = eval_postfix(input).unwrap_err();
        assert!(matches!(err, CalcError::NotEnoughOperands));
    }
}
