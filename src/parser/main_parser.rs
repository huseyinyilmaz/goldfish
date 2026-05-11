use nom::{bytes::tag, IResult, Parser};

use super::{
    add::make_add_parser, append::make_append_parser, cas::make_cas_parser, command::Command,
    decr::make_decr_parser, delete::make_delete_parser, flush_all::make_flush_all_parser,
    gat::make_gat_parser, gat::make_gats_parser, get::make_get_parser, get::make_gets_parser,
    incr::make_incr_parser, prepend::make_prepend_parser, quit::make_quit_parser,
    replace::make_replace_parser, set::make_set_parser, stats::make_stats_parser,
    touch::make_touch_parser, version::make_version_parser,
};

fn make_malformed_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn malformed_parser(input: &[u8]) -> IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (remaining, _) = nom::branch::alt((
            tag("incr"),
            tag("decr"),
            tag("flush_all"),
            tag("gats"),
            tag("gat"),
            tag("gets"),
            tag("get"),
            tag("touch"),
            tag("delete"),
            tag("stats"),
            tag("quit"),
            tag("version"),
        ))
        .parse(input)?;
        if !remaining.is_empty()
            && remaining[0] != b' '
            && remaining[0] != b'\r'
            && remaining[0] != b'\n'
        {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            )));
        }
        let (input, _) = nom::bytes::complete::take_till(|c| c == b'\r' || c == b'\n')(remaining)?;
        let (input, _) = nom::character::complete::line_ending(input)?;
        Ok((input, Command::Malformed))
    }
    malformed_parser
}

pub(crate) const COMMAND_KEYWORD_PREFIXES: &[&[u8]] = &[
    b"set ",
    b"add ",
    b"replace ",
    b"append ",
    b"prepend ",
    b"cas ",
    b"get ",
    b"gets ",
    b"gat ",
    b"gats ",
    b"delete ",
    b"incr ",
    b"decr ",
    b"touch ",
    b"flush_all",
    b"stats",
    b"version",
    b"quit",
];

pub(crate) fn starts_with_command_keyword(buf: &[u8]) -> bool {
    COMMAND_KEYWORD_PREFIXES.iter().any(|p| buf.starts_with(p))
}

fn make_cannotparse_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    |input: &'a [u8]| {
        if starts_with_command_keyword(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }
        if let Some(pos) = input.windows(2).position(|w| w == b"\r\n") {
            Ok((
                &input[pos + 2..],
                Command::CannotParse("Cannot Parse Input".to_owned()),
            ))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )))
        }
    }
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
        make_incr_parser(),
        make_decr_parser(),
        make_flush_all_parser(),
        make_replace_parser(),
        make_set_parser(),
        make_cas_parser(),
        make_gats_parser(),
        make_gat_parser(),
        make_gets_parser(),
        make_get_parser(),
        make_touch_parser(),
        make_stats_parser(),
        make_malformed_parser(),
        make_cannotparse_parser(),
    ))
}
