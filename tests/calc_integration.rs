use calculator::calc::{error::CalcError, evaluate};

#[test]
fn test_multiple_cases() {
    let cases = vec![
        ("1 + 1", 2.0),
        ("2 * 3", 6.0),
        ("10 / 2", 5.0),
        ("1 + 2 * 3", 7.0),
        ("(2 + 3) * 4", 20.0),
        ("(5 + 5) * (10 - 5) / 10", 5.0),
    ];

    for (input, expected) in cases {
        let res = evaluate(input).unwrap();
        assert_eq!(res, expected, "failed on input: {}", input);
    }
}

#[test]
fn test_bad_token() {
    let err = evaluate("1 + a").unwrap_err();
    assert!(matches!(err, CalcError::BadToken('a')));
}

#[test]
fn test_mismatched_parens() {
    let err = evaluate("(1 + 2").unwrap_err();
    assert!(matches!(err, CalcError::MismatchedParens));
}

#[test]
fn test_division_by_zero() {
    let err = evaluate("10 / 0").unwrap_err();
    assert!(matches!(err, CalcError::DivisionByZero));
}

#[test]
fn test_empty_input() {
    let err = evaluate("").unwrap_err();
    assert!(matches!(err, CalcError::NotEnoughOperands));
}

#[test]
fn test_only_number() {
    let res = evaluate("42").unwrap();
    assert_eq!(res, 42.0);
}

#[test]
fn test_spaces_everywhere() {
    let res = evaluate("   3   +   4   *  2 ").unwrap();
    assert_eq!(res, 11.0);
}

#[test]
fn test_nested_expression() {
    let res = evaluate("(2 + (3 * (4 + 1)))").unwrap();
    assert_eq!(res, 17.0);
}
