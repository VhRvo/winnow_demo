use winnow::combinator::preceded;
use winnow::token::take_while;
use winnow::{PResult, Parser};

fn parse_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    "0x".parse_next(input)
}

fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., (('0'..='9'), ('A'..='F'), ('a'..='f'))).parse_next(input)
}

#[test]
fn test1() {
    let mut input = "0x1a2b Hello";
    let prefix = parse_prefix.parse_next(&mut input).unwrap();
    let digits = parse_digits.parse_next(&mut input).unwrap();
    assert_eq!(prefix, "0x");
    assert_eq!(digits, "1a2b");
    assert_eq!(input, " Hello");
}

#[test]
fn test2() {
    let mut input = "0x1a2b Hello";
    let digits = preceded(parse_prefix, parse_digits)
        .parse_next(&mut input)
        .unwrap();
    // assert_eq!(prefix, "0x");
    assert_eq!(digits, "1a2b");
    assert_eq!(input, " Hello");
}
