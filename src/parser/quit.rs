use super::command::Command;
use nom::{
    bytes::tag, character::complete::{line_ending, space0}, sequence::{preceded, terminated}, IResult, Parser
};

/**
Parses following message: `b"quit/r/n"`
*/
pub fn make_quit_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command<'a>, Error = nom::error::Error<&'a [u8]>> {
    terminated(tag("quit"), line_ending).map(|_| Command::Quit)
}


