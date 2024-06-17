use winnow::combinator::{alt, fail, opt};
use winnow::stream::Stream;
use winnow::token::{one_of, take, take_while};
use winnow::{dispatch, PResult, Parser};

use crate::common::{parse_bin_digits, parse_dec_digits, parse_hex_digits, parse_oct_digits};

// fn parse_digits<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
//     let start = input.checkpoint();
//     if let Ok(output) = ("0b", parse_bin_digits).parse_next(input) {
//         return Ok(output);
//     }
//
//     input.reset(&start);
//     if let Ok(output) = ("0o", parse_oct_digits).parse_next(input) {
//         return Ok(output);
//     }
//
//     input.reset(&start);
//     if let Ok(output) = ("0d", parse_dec_digits).parse_next(input) {
//         return Ok(output);
//     }
//
//     input.reset(&start);
//     ("0x", parse_hex_digits).parse_next(input)
// }

// fn parse_digits<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
// if let Some(output) = opt(("0b", parse_bin_digits)).parse_next(input)? {
//     Ok(output)
// } else if let Some(output) = opt(("0o", parse_oct_digits)).parse_next(input)? {
//     Ok(output)
// } else if let Some(output) = opt(("0d", parse_dec_digits)).parse_next(input)? {
//     Ok(output)
// } else {
//     ("0x", parse_hex_digits).parse_next(input)
// }

// alt((
//     ("0b", parse_bin_digits),
//     ("0o", parse_oct_digits),
//     ("0d", parse_dec_digits),
//     ("0x", parse_hex_digits),
// ))
// .parse_next(input)
// }

fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    dispatch!(take(2_usize);
        "0b" => parse_bin_digits,
        "0o" => parse_oct_digits,
        "0d" => parse_dec_digits,
        "0x" => parse_hex_digits,
        _ => fail,
    )
    .parse_next(input)
}

#[test]
fn test1() {
    let mut input = "0x1a2b Hello";
    let (digits) = parse_digits.parse_next(&mut input).unwrap();

    assert_eq!(input, " Hello");
    // assert_eq!(prefix, "0x");
    assert_eq!(digits, "1a2b");
    assert!(parse_digits(&mut "ghiWorld").is_err());
}
