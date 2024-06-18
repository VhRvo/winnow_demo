use winnow::error::ErrorKind;
use winnow::error::{ErrMode, InputError};
use winnow::Parser;

#[test]
fn test() {
    use winnow::ascii::digit1;
    use winnow::token::take;

    let mut digits = take(5u8).and_then(digit1);

    assert_eq!(digits.parse_peek("12345"), Ok(("", "12345")));
    assert_eq!(digits.parse_next(&mut "12345"), Ok("12345"));
    assert_eq!(digits.parse_peek("123ab"), Ok(("", "123")));
    assert_eq!(
        digits.parse_peek("123"),
        Err(ErrMode::Backtrack(InputError::new("123", ErrorKind::Slice)))
    );
}
