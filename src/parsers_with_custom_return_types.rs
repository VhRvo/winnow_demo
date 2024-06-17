use winnow::Parser;

use crate::common::parse_digits;

#[test]
fn test1() {
    let mut input = "0d1024 Hello";

    let output = parse_digits.parse_next(&mut input).unwrap();
    assert_eq!(input, " Hello");
    assert_eq!(output, 1024);

    assert!(parse_digits(&mut "Z").is_err());
}
