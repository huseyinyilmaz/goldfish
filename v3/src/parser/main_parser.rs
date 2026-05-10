use nom::Parser;

use super::{
    add::make_add_parser, append::make_append_parser, command::Command, delete::make_delete_parser,
    get::make_get_parser, prepend::make_prepend_parser, quit::make_quit_parser,
    replace::make_replace_parser, set::make_set_parser, version::make_version_parser,
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
        make_delete_parser(),
        make_add_parser(),
        make_append_parser(),
        make_prepend_parser(),
        make_replace_parser(),
        make_set_parser(),
        make_get_parser(),
        make_cannotparse_parser(),
    ))
}
