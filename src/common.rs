use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use winnow::combinator::{alt, cut_err, fail};
use winnow::error::{ContextError, ParseError, StrContext, StrContextValue};
use winnow::Parser;
use winnow::PResult;
use winnow::token::take_while;

pub fn parse_bin_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., '0'..='1').parse_next(input)
}

pub fn parse_oct_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., '0'..='7').parse_next(input)
}

pub fn parse_dec_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., '0'..='9').parse_next(input)
}

pub fn parse_hex_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., ('0'..='9', 'A'..='F', 'a'..='f')).parse_next(input)
}

pub fn parse_digits(input: &mut &str) -> PResult<usize> {
    alt((
        ("0b", cut_err(parse_bin_digits))
            .context(StrContext::Label("digit"))
            .context(StrContext::Expected(StrContextValue::Description("binary")))
            .try_map(|(_, str)| usize::from_str_radix(str, 2)),
        ("0o", cut_err(parse_oct_digits))
            .context(StrContext::Label("digit"))
            .context(StrContext::Expected(StrContextValue::Description("octal")))
            .try_map(|(_, str)| usize::from_str_radix(str, 8)),
        ("0d", cut_err(parse_dec_digits))
            .context(StrContext::Label("digit"))
            .context(StrContext::Expected(StrContextValue::Description(
                "decimal",
            )))
            .try_map(|(_, str)| usize::from_str_radix(str, 10)),
        ("0x", cut_err(parse_hex_digits))
            .context(StrContext::Label("digit"))
            .context(StrContext::Expected(StrContextValue::Description(
                "hexadecimal",
            )))
            .try_map(|(_, str)| usize::from_str_radix(str, 16)),
        fail.context(StrContext::Label("radix prefix"))
            .context(StrContext::Expected(StrContextValue::StringLiteral("0b")))
            .context(StrContext::Expected(StrContextValue::StringLiteral("0o")))
            .context(StrContext::Expected(StrContextValue::StringLiteral("0d")))
            .context(StrContext::Expected(StrContextValue::StringLiteral("0x"))),
    ))
    .parse_next(input)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hex(pub usize);

impl std::str::FromStr for Hex {
    type Err = HexError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_digits
            .map(Hex)
            .parse(input)
            .map_err(|err| HexError::from_parse(err, input))
    }
}

#[derive(Debug)]
pub struct HexError {
    message: String,
    span: Range<usize>,
    input: String,
}

impl HexError {
    fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_owned();
        let start = error.offset();
        let end = (start + 1..)
            .find(|err| input.is_char_boundary(*err))
            .unwrap_or(start);
        Self {
            message,
            span: start..end,
            input,
        }
    }
}

impl Display for HexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = annotate_snippets::Level::Error
            .title(&self.message)
            .snippet(
                annotate_snippets::Snippet::source(&self.input)
                    .fold(true)
                    .annotation(annotate_snippets::Level::Error.span(self.span.clone())),
            );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl Error for HexError {}
