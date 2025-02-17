use super::command::Command;
use nom::{
    bytes::tag, character::complete::{line_ending, space0}, sequence::{preceded, terminated}, IResult, Parser
};

// quit/r/n
pub fn make_quit_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    terminated(preceded(space0, tag("quit")), line_ending).map(|_| Command::Quit)
}


