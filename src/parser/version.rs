use super::command::Command;
use nom::{
    bytes::tag,
    character::complete::line_ending,
    sequence::terminated,
    Parser,
};
/**
Parses following message: `b"version/r/n"`
*/
pub fn make_version_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    terminated(tag("version"), line_ending).map(|_| Command::Version)
}
