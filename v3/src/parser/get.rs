use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1},
    IResult, Parser,
};

/**
Parses following messages:
`b"get <key>\r\n"
*/
pub fn make_get_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn get_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("get")(input)?;
        let (input, _) = space1(input)?;
        let (input, key_bytestring) = is_not(" \r\n\t")(input)?;
        let (input, _) = line_ending(input)?;
        let key = key_bytestring.to_vec();
        let command: Command = Command::Get { key };
        IResult::Ok((input, command))
    }
    get_parser
}
