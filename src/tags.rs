use winnow::error::ErrMode;
use winnow::error::ErrorKind;
use winnow::error::ParserError;
use winnow::stream::Stream;
use winnow::{PResult, Parser};
// use winnow::token::literal;

fn parse_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let expected = "0x";
    if input.len() < expected.len() {
        Err(ErrMode::from_error_kind(input, ErrorKind::Slice))
    } else {
        let actual = input.next_slice(expected.len());
        if actual != expected {
            Err(ErrMode::from_error_kind(input, ErrorKind::Verify))
        } else {
            Ok(actual)
        }
    }
    // literal(expected).parse_next(input)
    // let mut expected = "0x";
    // expected.parse_next(input)
    // "0x".parse_next(input)
}

#[test]
fn test() {
    let mut input = "0x1a2b Hello";

    let output = parse_prefix.parse_next(&mut input).unwrap();
    assert_eq!(input, "1a2b Hello");
    assert_eq!(output, "0x");
    assert!(parse_prefix.parse_next(&mut "0o123").is_err());
}
