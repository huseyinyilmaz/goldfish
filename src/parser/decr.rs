use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1, u64},
    combinator::opt,
    IResult, Parser,
};

pub fn make_decr_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn decr_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("decr")(input)?;
        let (input, _) = space1(input)?;
        let (input, key_bytestring) = is_not(" \r\n")(input)?;
        let (input, _) = space1(input)?;
        let (input, delta) = u64(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, noreply_option) = opt(tag("noreply")).parse(input)?;
        let noreply = noreply_option.is_some();
        let (input, _) = line_ending(input)?;

        let key = key_bytestring.to_vec();
        let command: Command = Command::Decr {
            key,
            delta,
            noreply,
        };

        IResult::Ok((input, command))
    }
    decr_parser
}
