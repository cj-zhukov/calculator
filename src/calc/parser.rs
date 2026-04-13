use super::token::Token;

pub fn to_postfix(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens.reverse();

    let mut output = Vec::new();
    let mut stack = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            Token::Number(_) => output.push(token),

            Token::Op(_) => {
                while let Some(top) = stack.last() {
                    if matches!(top, Token::Op(_)) && top >= &token {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }

            Token::Bracket('(') => stack.push(token),

            Token::Bracket(')') => {
                while let Some(top) = stack.last() {
                    if *top == Token::Bracket('(') {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.pop(); // remove '('
            }

            _ => {}
        }
    }

    while let Some(token) = stack.pop() {
        output.push(token);
    }

    output
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

    fn lpar() -> Token {
        Token::Bracket('(')
    }

    fn rpar() -> Token {
        Token::Bracket(')')
    }

    #[test]
    fn postfix_simple_add() {
        let input = vec![n(1), op(Operator::Add), n(2)];
        let res = to_postfix(input);
        assert_eq!(res, vec![n(1), n(2), op(Operator::Add)]);
    }

    #[test]
    fn postfix_precedence() {
        // 1 + 2 * 3
        let input = vec![n(1), op(Operator::Add), n(2), op(Operator::Mul), n(3)];
        let res = to_postfix(input);

        // expected: 1 2 3 * +
        assert_eq!(
            res,
            vec![n(1), n(2), n(3), op(Operator::Mul), op(Operator::Add),]
        );
    }

    #[test]
    fn postfix_parentheses() {
        // (1 + 2) * 3
        let input = vec![
            lpar(),
            n(1),
            op(Operator::Add),
            n(2),
            rpar(),
            op(Operator::Mul),
            n(3),
        ];
        let res = to_postfix(input);

        // expected: 1 2 + 3 *
        assert_eq!(
            res,
            vec![n(1), n(2), op(Operator::Add), n(3), op(Operator::Mul),]
        );
    }

    #[test]
    fn postfix_left_associative() {
        // 10 - 5 - 2  => (10 - 5) - 2
        let input = vec![n(10), op(Operator::Sub), n(5), op(Operator::Sub), n(2)];
        let res = to_postfix(input);

        // expected: 10 5 - 2 -
        assert_eq!(
            res,
            vec![n(10), n(5), op(Operator::Sub), n(2), op(Operator::Sub),]
        );
    }

    #[test]
    fn postfix_nested_parentheses() {
        // (1 + (2 * 3))
        let input = vec![
            lpar(),
            n(1),
            op(Operator::Add),
            lpar(),
            n(2),
            op(Operator::Mul),
            n(3),
            rpar(),
            rpar(),
        ];
        let res = to_postfix(input);

        // expected: 1 2 3 * +
        assert_eq!(
            res,
            vec![n(1), n(2), n(3), op(Operator::Mul), op(Operator::Add),]
        );
    }

    #[test]
    fn postfix_single_number() {
        let input = vec![n(42)];
        let res = to_postfix(input);
        assert_eq!(res, vec![n(42)]);
    }

    #[test]
    fn postfix_empty() {
        let input = vec![];
        let res = to_postfix(input);
        assert!(res.is_empty());
    }
}
