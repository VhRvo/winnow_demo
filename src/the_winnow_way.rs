use winnow::combinator::trace;
use winnow::PResult;
use winnow::Parser;

pub fn do_nothing_parser<'s>(input: &mut &'s str) -> PResult<&'s str> {
    trace("do_nothing_parser", |_i: &mut _| Ok("")).parse_next(input)
}

#[test]
fn test() {
    let mut input = "0x1a2b Hello";
    let output = do_nothing_parser.parse_next(&mut input).unwrap();
    assert_eq!(input, "0x1a2b Hello");
    assert_eq!(output, "");
}
