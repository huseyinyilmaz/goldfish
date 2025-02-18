use std::str::{from_utf8, FromStr};
use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag}, character::complete::{digit1, space1}, combinator::rest, sequence::{preceded, separated_pair, terminated}, IResult, Parser
};

/**
Parses following messages:
`b"set <key> <flags(int)> <expire(expiration seconds)> <value bytes> [noreply]\r\n<value bytes>\r\n"`
*/
pub fn make_set_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command<'a>, Error = nom::error::Error<&'a [u8]>> {
    fn set_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("set")(input)?; 
        let (input, _) = space1(input)?; 
        let (input, key) = is_not(" ")(input)?; 
        let (input, _) = space1(input)?; 
        let (input, flags_str) = digit1(input)?; 
        let (input, _) = space1(input)?; 
        let (input, timeout_str) = digit1(input)?; 
        let (input, _) = space1(input)?; 
        let (input, value_size_str) = digit1(input)?; 
        let (input,  _) = rest(input)?; 

        let flags = FromStr::from_str(from_utf8(flags_str).unwrap()).unwrap();
        let timeout = FromStr::from_str(from_utf8(timeout_str).unwrap()).unwrap();
        let value_size = FromStr::from_str(from_utf8(value_size_str).unwrap()).unwrap();

        let command: Command = Command::Set{ key, flags , timeout, value_size };

        return IResult::Ok((input, command));
    }
    return set_parser;
    }
//     // separated_pair(alphanumeric1, space1, rest)
//     preceded(tag("set"), rest)
//     // terminated(
//     //     preceded(tag("set"), rest),
//     //     line_ending,
//     // )
//     .map(|_| Command::Version)
// }
