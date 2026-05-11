use std::str::{from_utf8, FromStr};

use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag, take},
    character::complete::{digit1, i64, line_ending, space1},
    combinator::opt,
    IResult, Parser,
};

pub fn make_append_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn append_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("append")(input)?;
        let (input, _) = space1(input)?;
        let (input, key_bytestring) = is_not(" ")(input)?;
        let (input, _) = space1(input)?;
        let (input, flags_str) = digit1(input)?;
        let (input, _) = space1(input)?;
        let (input, timeout) = i64(input)?;
        let (input, _) = space1(input)?;
        let (input, value_size_str) = digit1(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, noreply_option) = opt(tag("noreply")).parse(input)?;
        let noreply = noreply_option.is_some();

        let (input, _) = line_ending(input)?;
        let value_size: u32 = FromStr::from_str(from_utf8(value_size_str).unwrap()).unwrap();
        let (input, value_bytestring) = take(value_size)(input)?;
        let (input, _) = line_ending(input)?;

        let flags = FromStr::from_str(from_utf8(flags_str).unwrap()).unwrap();
        let key = key_bytestring.to_vec();
        let value = value_bytestring.to_vec();
        let command: Command = Command::Append {
            key,
            flags,
            timeout,
            noreply,
            value_size,
            value,
        };

        IResult::Ok((input, command))
    }
    append_parser
}
