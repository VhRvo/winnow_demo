use winnow::{Parser, PResult};
use winnow::combinator::{opt, repeat, separated, terminated};

use crate::common::parse_digits;

#[allow(unused)]
fn parse_list_1(input: &mut &str) -> PResult<Vec<usize>> {
    let mut list = Vec::new();
    while let Some(output) = opt(terminated(parse_digits, opt(','))).parse_next(input)? {
        list.push(output);
    }
    Ok(list)
}

#[allow(unused)]
fn parse_list_2(input: &mut &str) -> PResult<Vec<usize>> {
    repeat(0.., terminated(parse_digits, opt(','))).parse_next(input)
}

fn recognize_list<'s>(input: &mut &'s str) -> PResult<&'s str> {
    parse_list.recognize().parse_next(input)
}

fn parse_list(input: &mut &str) -> PResult<Vec<usize>> {
    separated(0.., parse_digits, ",").parse_next(input)
}

#[test]
fn test1() {
    let mut input = "0x1a2b,0x3c4d,0x5e6f Hello";
    let digits = parse_list.parse_next(&mut input).unwrap();
    assert_eq!(input, " Hello");
    assert_eq!(digits, vec![0x1a2b, 0x3c4d, 0x5e6f]);

    assert!(parse_digits(&mut "ghiWorld").is_err());
}

#[test]
fn test2() {
    let mut input = "0x1a2b,0x3c4d,0x5e6f Hello";
    let digits = recognize_list.parse_next(&mut input).unwrap();
    assert_eq!(input, " Hello");
    assert_eq!(digits, "0x1a2b,0x3c4d,0x5e6f");

    assert!(parse_digits(&mut "ghiWorld").is_err());
}
