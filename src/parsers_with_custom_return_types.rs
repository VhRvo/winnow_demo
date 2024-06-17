use winnow::ascii::digit1;
use winnow::combinator::fail;
use winnow::token::take;
use winnow::{dispatch, PResult, Parser};

use crate::common::{parse_bin_digits, parse_dec_digits, parse_hex_digits, parse_oct_digits};

// fn parse_digits(input: &mut &str) -> PResult<usize> {
//     digit1.parse_to().parse_next(input)
// }
fn parse_digits(input: &mut &str) -> PResult<usize> {
    dispatch!(take(2_usize);
        "0b" => parse_bin_digits.try_map(|str| usize::from_str_radix(str, 2)),
        "0o" => parse_oct_digits.try_map(|str| usize::from_str_radix(str, 8)),
        "0d" => parse_dec_digits.try_map(|str| usize::from_str_radix(str, 10)),
        "0x" => parse_hex_digits.try_map(|str| usize::from_str_radix(str, 16)),
        _ => fail,
    )
    .parse_next(input)
}

#[test]
fn test1() {
    let mut input = "0d1024 Hello";

    let output = parse_digits.parse_next(&mut input).unwrap();
    assert_eq!(input, " Hello");
    assert_eq!(output, 1024);

    assert!(parse_digits(&mut "Z").is_err());
}
