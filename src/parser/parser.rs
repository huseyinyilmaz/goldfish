use nom::{
    // alphanumeric,
    bytes::complete, character::complete::{alphanumeric1, anychar, space1}, combinator::rest, error::{self, Error}, multi::many1, IResult, Parser
};
use tokio::sync::watch;

use super::{command::Command, quit::make_quit_parser, version::make_version_parser};


fn make_cannotparse_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    nom::combinator::rest.map(|_| Command::CannotParse("Cannot Parse Input".to_owned()))
}


pub fn make_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    nom::branch::alt(
        (
            make_quit_parser(), 
            make_version_parser(),
            make_cannotparse_parser(),
)
    )
}

// pub fn parse_command(raw_input: &[u8]) -> Command {
//     match make_parser().parse(raw_input) {
//         Ok((_buf, command)) => command,
//         _ => Command::CannotParse("Cannot Parse Input".to_owned()),
//     }
// }
