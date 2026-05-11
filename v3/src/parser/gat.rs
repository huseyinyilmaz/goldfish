use super::command::Command;
use super::get::parse_keys;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending, space1},
    IResult, Parser,
};

pub fn make_gat_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn gat_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("gat")(input)?;
        let (input, _) = space1(input)?;
        let (input, timeout) = i64(input)?;
        let (input, _) = space1(input)?;
        let (input, keys) = parse_keys(input)?;
        let (input, _) = line_ending(input)?;
        let command: Command = Command::Gat { timeout, keys };
        Ok((input, command))
    }
    gat_parser
}

pub fn make_gats_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn gats_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("gats")(input)?;
        let (input, _) = space1(input)?;
        let (input, timeout) = i64(input)?;
        let (input, _) = space1(input)?;
        let (input, keys) = parse_keys(input)?;
        let (input, _) = line_ending(input)?;
        let command: Command = Command::Gats { timeout, keys };
        Ok((input, command))
    }
    gats_parser
}
