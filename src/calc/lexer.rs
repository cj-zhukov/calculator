//! # Lexer
//!
//! This module is responsible for converting a raw input string into a sequence
//! of tokens (`Vec<Token>`).
//!
//! It performs the first stage of the calculation pipeline:
//!
//! ```text
//! &str → Vec<Token> → Vec<Token> → f32
//!  |        |             |        |
//! str     tokens       postfix   result
//!  ↑        ↑             ↑
//! lexer   parser      evaluator
//! ```
//!
//! ## Responsibilities
//!
//! - Parse numeric literals (including floating point numbers like `12.5`)
//! - Recognize arithmetic operators (`+`, `-`, `*`, `/`)
//! - Handle parentheses (`(`, `)`)
//! - Ignore whitespace
//! - Validate balanced parentheses
//! - Detect invalid characters
//!
//! ## What it does NOT do
//!
//! - Does not validate expression correctness (e.g. `1 2` is allowed)
//! - Does not handle operator precedence
//! - Does not evaluate expressions
//!
//! These responsibilities belong to later stages (parser and evaluator).
//!
//! ## Notes
//!
//! - Numbers are parsed using a buffer and converted to `f32`
//! - Invalid number formats (e.g. `1..2`) will result in an error during parsing
//! - Parentheses are tracked during tokenization to detect mismatches early

use super::error::CalcError;
use super::token::*;

pub fn tokenize(expr: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut parens = Vec::new();
    let mut number_buf = String::new();

    let flush_number = |buf: &mut String, tokens: &mut Vec<Token>| -> Result<(), CalcError> {
        if !buf.is_empty() {
            let n: f32 = buf.parse().map_err(|_| CalcError::BadToken('?'))?;
            tokens.push(Token::Number(n));
            buf.clear();
        }
        Ok(())
    };

    for c in expr.chars() {
        match c {
            '0'..='9' | '.' => {
                number_buf.push(c);
            }

            '+' | '-' | '*' | '/' => {
                flush_number(&mut number_buf, &mut tokens)?;
                let op = match c {
                    '+' => Operator::Add,
                    '-' => Operator::Sub,
                    '*' => Operator::Mul,
                    '/' => Operator::Div,
                    _ => unreachable!(),
                };
                tokens.push(Token::Op(op));
            }

            '(' => {
                flush_number(&mut number_buf, &mut tokens)?;
                tokens.push(Token::Bracket(Bracket::Open));
                parens.push(c);
            }

            ')' => {
                flush_number(&mut number_buf, &mut tokens)?;
                tokens.push(Token::Bracket(Bracket::Close));
                if parens.pop().is_none() {
                    return Err(CalcError::MismatchedParens);
                }
            }

            ' ' | '\n' => {
                flush_number(&mut number_buf, &mut tokens)?;
            }

            _ => return Err(CalcError::BadToken(c)),
        }
    }

    // flush last number
    flush_number(&mut number_buf, &mut tokens)?;

    if !parens.is_empty() {
        return Err(CalcError::MismatchedParens);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_expression() {
        let tokens = tokenize("1 + 2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
            ]
        );
    }

    #[test]
    fn tokenize_multi_digit_number() {
        let tokens = tokenize("123 + 45").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123.),
                Token::Op(Operator::Add),
                Token::Number(45.),
            ]
        );
    }

    #[test]
    fn tokenize_all_operators() {
        let tokens = tokenize("1 + 2 - 3 * 4 / 5").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
                Token::Op(Operator::Sub),
                Token::Number(3.),
                Token::Op(Operator::Mul),
                Token::Number(4.),
                Token::Op(Operator::Div),
                Token::Number(5.),
            ]
        );
    }

    #[test]
    fn tokenize_parentheses() {
        let tokens = tokenize("(1 + 2)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Bracket(Bracket::Open),
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
                Token::Bracket(Bracket::Close),
            ]
        );
    }

    #[test]
    fn error_unexpected_closing_paren() {
        let err = tokenize("1 + 2)").unwrap_err();
        assert!(matches!(err, CalcError::MismatchedParens));
    }

    #[test]
    fn error_missing_closing_paren() {
        let err = tokenize("(1 + 2").unwrap_err();
        assert!(matches!(err, CalcError::MismatchedParens));
    }

    #[test]
    fn error_bad_token() {
        let err = tokenize("1 + a").unwrap_err();
        assert_eq!(err, CalcError::BadToken('a'));
    }

    #[test]
    fn tokenize_ignores_whitespace() {
        let tokens = tokenize("  1   +   2 \n").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
            ]
        );
    }

    #[test]
    fn tokenize_empty_input() {
        let tokens = tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_numbers_separated_by_space() {
        let tokens = tokenize("1 2").unwrap();
        assert_eq!(tokens, vec![Token::Number(1.0), Token::Number(2.0),]);
    }
}
