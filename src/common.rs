use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

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
