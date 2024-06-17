use winnow::ascii::hex_digit1;
use winnow::{PResult, Parser};
// use winnow::token::{one_of, take_while};

fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    // one_of(('0'..='9', 'a'..='f', 'A'..='F')).parse_next(input)
    // take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F')).parse_next(input)
    hex_digit1.parse_next(input)
}

#[test]
fn test() {
    let mut input = "1a2b Hello";
    let output = parse_digits.parse_next(&mut input).unwrap();
    // assert_eq!(input, "a2b Hello");
    // assert_eq!(output, '1');
    assert_eq!(input, " Hello");
    assert_eq!(output, "1a2b");

    assert!(parse_digits.parse_next(&mut "Z").is_err());
}
