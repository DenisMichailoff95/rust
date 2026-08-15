use level6_task28_parse_int::*;

#[test]
fn test_parse_int() {
    assert_eq!(parse_int("42"), Ok(42));
    assert_eq!(parse_int("-10"), Ok(-10));
    assert!(parse_int("abc").is_err());
}
