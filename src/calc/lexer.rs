//! # Lexer
//!
//! This module converts a raw expression string into a sequence of tokens
//! (`Vec<Token>`).
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
//! - Parse numeric literals (`1`, `42`, `3.14`)
//! - Recognize arithmetic operators (`+`, `-`, `*`, `/`)
//! - Recognize parentheses (`(`, `)`)
//! - Ignore whitespace
//! - Validate balanced parentheses
//! - Detect invalid characters and malformed numbers
//!
//! ## What it does NOT do
//!
//! - Does not validate full expression correctness
//!   (`1 2` is tokenized successfully)
//! - Does not handle operator precedence
//! - Does not evaluate expressions
//!
//! These responsibilities belong to later stages
//! (parser and evaluator).
//!
//! ## Number Parsing
//!
//! Numbers are built incrementally while scanning characters.
//!
//! Integer digits shift existing digits left:
//!
//! ```text
//! 12
//!
//! 0 * 10 + 1 = 1
//! 1 * 10 + 2 = 12
//! ```
//!
//! Fractional digits are added using decreasing multipliers:
//!
//! ```text
//! 3.14
//!
//! integer part:
//! 0 * 10 + 3 = 3
//!
//! fractional part:
//! 3 + 1 * 0.1  = 3.1
//! 3.1 + 4 * 0.01 = 3.14
//! ```
//!
//! This avoids temporary `String` allocations and parsing via
//! `str::parse::<f32>()`.
//!
//! ## Example
//!
//! Input:
//!
//! ```text
//! (12.5 + 3) * 2
//! ```
//!
//! Output:
//!
//! ```text
//! [
//!     Bracket(Open),
//!     Number(12.5),
//!     Op(Add),
//!     Number(3.0),
//!     Bracket(Close),
//!     Op(Mul),
//!     Number(2.0),
//! ]
//! ```
//!
//! ## Errors
//!
//! The lexer may return:
//!
//! - `BadToken(char)`
//! - `MismatchedParens`
//!
//! ## Notes
//!
//! - Parentheses are validated during tokenization
//! - Multiple decimal points in a number (`1..2`) are rejected
//! - Lexer output is consumed by the parser stage

use super::error::CalcError;
use super::token::*;

pub fn tokenize(expr: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::with_capacity(expr.len());
    let mut paren_depth = 0;
    // Current number being built incrementally.
    let mut number = 0.0;
    // Tracks whether we are currently parsing a number.
    let mut building_number = false;
    // True after encountering '.' in a number.
    let mut in_fraction = false;
    // Decimal place multiplier: 0.1 → 0.01 → 0.001 ...
    let mut fraction_multiplier = 0.1;

    // Push completed number into tokens and reset parser state.
    let flush_number = |number: &mut f32,
                        building_number: &mut bool,
                        in_fraction: &mut bool,
                        fraction_multiplier: &mut f32,
                        tokens: &mut Vec<Token>| {
        if *building_number {
            tokens.push(Token::Number(*number));

            *number = 0.0;
            *building_number = false;
            *in_fraction = false;
            *fraction_multiplier = 0.1;
        }
    };

    for c in expr.chars() {
        match c {
            '0'..='9' => {
                let digit = (c as u8 - b'0') as f32;
                // Build decimal part: 0.1, 0.01, 0.001 ...
                if in_fraction {
                    number += digit * fraction_multiplier;
                    fraction_multiplier *= 0.1;
                } else {
                    // Shift existing digits left and append new digit.
                    number = number * 10.0 + digit;
                }
                building_number = true;
            }

            '.' => {
                // Reject multiple dots in the same number.
                if in_fraction {
                    return Err(CalcError::BadToken('.'));
                }
                in_fraction = true;
                building_number = true;
            }

            '+' | '-' | '*' | '/' => {
                flush_number(
                    &mut number,
                    &mut building_number,
                    &mut in_fraction,
                    &mut fraction_multiplier,
                    &mut tokens,
                );
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
                flush_number(
                    &mut number,
                    &mut building_number,
                    &mut in_fraction,
                    &mut fraction_multiplier,
                    &mut tokens,
                );
                tokens.push(Token::Bracket(Bracket::Open));
                paren_depth += 1;
            }

            ')' => {
                if paren_depth == 0 {
                    return Err(CalcError::MismatchedParens);
                }
                flush_number(
                    &mut number,
                    &mut building_number,
                    &mut in_fraction,
                    &mut fraction_multiplier,
                    &mut tokens,
                );
                tokens.push(Token::Bracket(Bracket::Close));
                paren_depth -= 1;
            }

            ' ' | '\n' => {
                flush_number(
                    &mut number,
                    &mut building_number,
                    &mut in_fraction,
                    &mut fraction_multiplier,
                    &mut tokens,
                );
            }

            _ => return Err(CalcError::BadToken(c)),
        }
    }

    // Push final number if expression ends with a digit.
    flush_number(
        &mut number,
        &mut building_number,
        &mut in_fraction,
        &mut fraction_multiplier,
        &mut tokens,
    );

    if paren_depth != 0 {
        return Err(CalcError::MismatchedParens);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_expression() -> Result<(), CalcError> {
        let tokens = tokenize("1 + 2")?;
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
            ]
        );
        Ok(())
    }

    #[test]
    fn tokenize_multi_digit_number() -> Result<(), CalcError> {
        let tokens = tokenize("123 + 45")?;
        assert_eq!(
            tokens,
            vec![
                Token::Number(123.),
                Token::Op(Operator::Add),
                Token::Number(45.),
            ]
        );
        Ok(())
    }

    #[test]
    fn tokenize_all_operators() -> Result<(), CalcError> {
        let tokens = tokenize("1 + 2 - 3 * 4 / 5")?;
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
        Ok(())
    }

    #[test]
    fn tokenize_parentheses() -> Result<(), CalcError> {
        let tokens = tokenize("(1 + 2)")?;
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
        Ok(())
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
    fn tokenize_ignores_whitespace() -> Result<(), CalcError> {
        let tokens = tokenize("  1   +   2 \n")?;
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.),
                Token::Op(Operator::Add),
                Token::Number(2.),
            ]
        );
        Ok(())
    }

    #[test]
    fn tokenize_empty_input() -> Result<(), CalcError> {
        let tokens = tokenize("")?;
        assert!(tokens.is_empty());
        Ok(())
    }

    #[test]
    fn tokenize_numbers_separated_by_space() -> Result<(), CalcError> {
        let tokens = tokenize("1 2")?;
        assert_eq!(tokens, vec![Token::Number(1.0), Token::Number(2.0),]);
        Ok(())
    }
}
