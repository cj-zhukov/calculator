use calculator::calc::{calculate, error::CalcError};

#[test]
fn test_multiple_cases() {
    let cases = vec![
        ("1 + 1", 2.0),
        ("1+1", 2.0),
        ("2 * 3", 6.0),
        ("10 / 2", 5.0),
        ("1 + 2 * 3", 7.0),
        ("(2 + 3) * 4", 20.0),
        ("(5 + 5) * (10 - 5) / 10", 5.0),
        ("(5.0 + 5.0) * (10.0 - 5.0) / 10.0", 5.0),
    ];

    for (input, expected) in cases {
        let res = calculate(input).unwrap();
        assert_eq!(res, expected, "failed on input: {}", input);
    }
}

#[test]
fn test_bad_token() {
    let err = calculate("1 + a").unwrap_err();
    assert!(matches!(err, CalcError::BadToken('a')));

    let err = calculate("1,1 + 2,1").unwrap_err();
    assert!(matches!(err, CalcError::BadToken(',')));
}

#[test]
fn test_mismatched_parens() {
    let err = calculate("(1 + 2").unwrap_err();
    assert!(matches!(err, CalcError::MismatchedParens));
}

#[test]
fn test_not_enough_operands() {
    let err = calculate("1 + ").unwrap_err();
    assert!(matches!(err, CalcError::NotEnoughOperands));
}

#[test]
fn test_division_by_zero() {
    let err = calculate("10 / 0").unwrap_err();
    assert!(matches!(err, CalcError::DivisionByZero));
}

#[test]
fn test_empty_input() {
    let err = calculate("").unwrap_err();
    assert!(matches!(err, CalcError::InvalidExpression));
}

#[test]
fn test_only_number() {
    let res = calculate("42").unwrap();
    assert_eq!(res, 42.0);
}

#[test]
fn test_spaces_everywhere() {
    let res = calculate("   3   +   4   *  2 ").unwrap();
    assert_eq!(res, 11.0);
}

#[test]
fn test_nested_expression() {
    let res = calculate("(2 + (3 * (4 + 1)))").unwrap();
    assert_eq!(res, 17.0);
}
