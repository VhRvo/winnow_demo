use winnow::error::ErrMode;
use winnow::error::ErrorKind;
use winnow::error::ParserError;
use winnow::stream::Stream;
use winnow::{PResult, Parser};
// use winnow::token::any;

fn parse_prefix(input: &mut &str) -> PResult<char> {
    let ch = input
        .next_token()
        .ok_or_else(|| ErrMode::from_error_kind(input, ErrorKind::Token))?;
    // let ch = any.parse_next(input)?;
    if ch != '0' {
        Err(ErrMode::from_error_kind(input, ErrorKind::Verify))
    } else {
        Ok(ch)
    }
    // any.verify(|ch| *ch == '0').parse_next(input)
    // '0'.parse_next(input)
}

#[test]
fn test() {
    let mut input = "0x1a2b Hello";

    let output = parse_prefix.parse_next(&mut input).unwrap();
    assert_eq!(input, "x1a2b Hello");
    assert_eq!(output, '0');
    assert!(parse_prefix.parse_next(&mut "d").is_err());
}
