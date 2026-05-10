use super::command::Command;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::opt,
    IResult, Parser,
};

pub fn make_flush_all_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn flush_all_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("flush_all")(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, delay) = opt(nom::character::complete::u64).parse(input)?;
        let (input, _) = opt(space1).parse(input)?;
        let (input, noreply_option) = opt(tag("noreply")).parse(input)?;
        let noreply = noreply_option.is_some();

        let (input, _) = line_ending(input)?;

        let command: Command = Command::FlushAll {
            delay: delay.unwrap_or(0),
            noreply,
        };

        IResult::Ok((input, command))
    }
    flush_all_parser
}
