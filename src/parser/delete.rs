use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1},
    combinator::opt,
    IResult, Parser,
};

pub fn make_delete_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn delete_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("delete")(input)?;
        let (input, _) = space1(input)?;
        let (input, key_bytestring) = is_not(" \r\n")(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, noreply_option) = opt(tag("noreply")).parse(input)?;
        let noreply = noreply_option.is_some();

        let (input, _) = line_ending(input)?;

        let key = key_bytestring.to_vec();
        let command: Command = Command::Delete { key, noreply };

        IResult::Ok((input, command))
    }
    delete_parser
}
