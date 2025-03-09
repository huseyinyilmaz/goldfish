use std::str::{from_utf8, FromStr};

use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag, take},
    character::complete::{digit1, line_ending, space1},
    combinator::opt,
    IResult, Parser,
};

/**
Parses following messages:
`b"set <key> <flags(int)> <expire(expiration seconds)> <value bytes> [noreply]\r\n<value bytes>\r\n"`
*/
pub fn make_set_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn set_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("set")(input)?;
        let (input, _) = space1(input)?;
        let (input, key_bytestring) = is_not(" ")(input)?;
        let (input, _) = space1(input)?;
        let (input, flags_str) = digit1(input)?;
        let (input, _) = space1(input)?;
        let (input, timeout_str) = digit1(input)?;
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
        let timeout = FromStr::from_str(from_utf8(timeout_str).unwrap()).unwrap();
        let key = key_bytestring.to_vec();
        let value = value_bytestring.to_vec();
        let command: Command = Command::Set {
            key,
            flags,
            timeout,
            noreply,
            value_size,
            value,
        };

        return IResult::Ok((input, command));
    }
    return set_parser;
}
