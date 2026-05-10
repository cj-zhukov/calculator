//! # Parser (Infix → Postfix)
//!
//! This module converts tokens from infix notation into postfix notation
//! (Reverse Polish Notation, RPN).
//!
//! It performs the second stage of the calculation pipeline:
//!
//! ```text
//! &str → Vec<Token> → Vec<Token> → f32
//!  |        |             |        |
//! str     tokens       postfix   result
//!  ↑        ↑             ↑
//! lexer   parser      evaluator
//! ```
//!
//! ## What this step does
//!
//! - Applies operator precedence (e.g. `*` before `+`)
//! - Resolves parentheses
//! - Converts the expression into a format that is easy to evaluate
//!
//! Example:
//!
//! ```text
//! Infix:   1 + 2 * 3
//! Postfix: 1 2 3 * +
//! ```
//!
//! ## Why postfix?
//!
//! Postfix notation eliminates the need for precedence rules during evaluation.
//! The resulting expression can be executed using a simple stack-based algorithm.
//!
//! ## Notes
//!
//! - Implements a variation of the Shunting Yard algorithm
//! - Assumes input tokens are valid and parentheses are balanced
//! - Output is consumed by the evaluator stage

use super::token::*;

pub fn to_postfix(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens.reverse();

    let mut output = Vec::with_capacity(tokens.len());
    let mut stack = Vec::with_capacity(tokens.len());

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

            Token::Bracket(Bracket::Open) => stack.push(token),

            Token::Bracket(Bracket::Close) => {
                while let Some(top) = stack.last() {
                    if *top == Token::Bracket(Bracket::Open) {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.pop(); // remove '('
            }
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

    fn n(x: f32) -> Token {
        Token::Number(x)
    }

    fn op(op: Operator) -> Token {
        Token::Op(op)
    }

    fn lpar() -> Token {
        Token::Bracket(Bracket::Open)
    }

    fn rpar() -> Token {
        Token::Bracket(Bracket::Close)
    }

    #[test]
    fn postfix_simple_add() {
        let input = vec![n(1.), op(Operator::Add), n(2.)];
        let res = to_postfix(input);
        assert_eq!(res, vec![n(1.), n(2.), op(Operator::Add)]);
    }

    #[test]
    fn postfix_precedence() {
        // 1 + 2 * 3
        let input = vec![n(1.), op(Operator::Add), n(2.), op(Operator::Mul), n(3.)];
        let res = to_postfix(input);

        // expected: 1 2 3 * +
        assert_eq!(
            res,
            vec![n(1.), n(2.), n(3.), op(Operator::Mul), op(Operator::Add),]
        );
    }

    #[test]
    fn postfix_parentheses() {
        // (1 + 2) * 3
        let input = vec![
            lpar(),
            n(1.),
            op(Operator::Add),
            n(2.),
            rpar(),
            op(Operator::Mul),
            n(3.),
        ];
        let res = to_postfix(input);

        // expected: 1 2 + 3 *
        assert_eq!(
            res,
            vec![n(1.), n(2.), op(Operator::Add), n(3.), op(Operator::Mul),]
        );
    }

    #[test]
    fn postfix_left_associative() {
        // 10 - 5 - 2  => (10 - 5) - 2
        let input = vec![n(10.), op(Operator::Sub), n(5.), op(Operator::Sub), n(2.)];
        let res = to_postfix(input);

        // expected: 10 5 - 2 -
        assert_eq!(
            res,
            vec![n(10.), n(5.), op(Operator::Sub), n(2.), op(Operator::Sub),]
        );
    }

    #[test]
    fn postfix_nested_parentheses() {
        // (1 + (2 * 3))
        let input = vec![
            lpar(),
            n(1.),
            op(Operator::Add),
            lpar(),
            n(2.),
            op(Operator::Mul),
            n(3.),
            rpar(),
            rpar(),
        ];
        let res = to_postfix(input);

        // expected: 1 2 3 * +
        assert_eq!(
            res,
            vec![n(1.), n(2.), n(3.), op(Operator::Mul), op(Operator::Add),]
        );
    }

    #[test]
    fn postfix_single_number() {
        let input = vec![n(42.)];
        let res = to_postfix(input);
        assert_eq!(res, vec![n(42.)]);
    }

    #[test]
    fn postfix_empty() {
        let input = vec![];
        let res = to_postfix(input);
        assert!(res.is_empty());
    }
}
