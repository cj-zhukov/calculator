use super::error::CalcError;
use super::token::*;

pub fn tokenize(expr: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut parens = Vec::new();

    for c in expr.chars() {
        match c {
            '0'..='9' => match tokens.last_mut() {
                Some(Token::Number(n)) => *n = *n * 10 + (c as u32 - 48),
                _ => {
                    tokens.push(Token::Number(c as u32 - 48));
                }
            },
            '+' => tokens.push(Token::Op(Operator::Add)),
            '-' => tokens.push(Token::Op(Operator::Sub)),
            '*' => tokens.push(Token::Op(Operator::Mul)),
            '/' => tokens.push(Token::Op(Operator::Div)),
            '(' => {
                tokens.push(Token::Bracket(Bracket::Open));
                parens.push(c);
            }
            ')' => {
                tokens.push(Token::Bracket(Bracket::Close));
                if parens.pop().is_none() {
                    return Err(CalcError::MismatchedParens);
                }
            }
            ' ' | '\n' => {}
            _ => return Err(CalcError::BadToken(c)),
        }
    }

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
            vec![Token::Number(1), Token::Op(Operator::Add), Token::Number(2),]
        );
    }

    #[test]
    fn tokenize_multi_digit_number() {
        let tokens = tokenize("123 + 45").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Op(Operator::Add),
                Token::Number(45),
            ]
        );
    }

    #[test]
    fn tokenize_all_operators() {
        let tokens = tokenize("1 + 2 - 3 * 4 / 5").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Op(Operator::Add),
                Token::Number(2),
                Token::Op(Operator::Sub),
                Token::Number(3),
                Token::Op(Operator::Mul),
                Token::Number(4),
                Token::Op(Operator::Div),
                Token::Number(5),
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
                Token::Number(1),
                Token::Op(Operator::Add),
                Token::Number(2),
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
            vec![Token::Number(1), Token::Op(Operator::Add), Token::Number(2),]
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
        assert_eq!(tokens, vec![Token::Number(12)]);
    }
}
