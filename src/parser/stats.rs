use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1},
    combinator::opt,
    IResult, Parser,
};

pub fn make_stats_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn stats_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("stats")(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, sub) = opt(is_not(" \r\n")).parse(input)?;

        let (input, _) = line_ending(input)?;

        let command: Command = Command::Stats {
            sub: sub.map(|s| s.to_vec()),
        };

        IResult::Ok((input, command))
    }
    stats_parser
}
