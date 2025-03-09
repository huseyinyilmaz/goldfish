use nom::Parser;

use super::{
    command::Command, get::make_get_parser, quit::make_quit_parser, set::make_set_parser, version::make_version_parser
};

fn make_cannotparse_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    nom::combinator::rest.map(|_| Command::CannotParse("Cannot Parse Input".to_owned()))
}

pub fn make_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    nom::branch::alt((
        make_quit_parser(),
        make_version_parser(),
        make_set_parser(),
        make_get_parser(),
        make_cannotparse_parser(),
    ))
}

