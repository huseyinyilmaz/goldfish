use super::command::Command;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1},
    multi::many0,
    sequence::preceded,
    Parser,
};

fn parse_keys(input: &[u8]) -> nom::IResult<&[u8], Vec<Vec<u8>>> {
    let (input, first_key) = is_not(" \r\n\t")(input)?;
    let (input, more_keys) = many0(preceded(space1, is_not(" \r\n\t"))).parse(input)?;
    let mut keys = vec![first_key.to_vec()];
    keys.extend(more_keys.into_iter().map(|k: &[u8]| k.to_vec()));
    Ok((input, keys))
}

/**
Parses following messages:
`b"get <key>\r\n"
`b"get <key1> <key2> ... <keyN>\r\n"
*/
pub fn make_get_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn get_parser(input: &[u8]) -> nom::IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("get")(input)?;
        let (input, _) = space1(input)?;
        let (input, keys) = parse_keys(input)?;
        let (input, _) = line_ending(input)?;
        let command: Command = Command::Get { keys };
        Ok((input, command))
    }
    get_parser
}

pub fn make_gets_parser<'a>(
) -> impl Parser<&'a [u8], Output = Command, Error = nom::error::Error<&'a [u8]>> {
    fn gets_parser(input: &[u8]) -> nom::IResult<&[u8], Command, nom::error::Error<&[u8]>> {
        let (input, _) = tag("gets")(input)?;
        let (input, _) = space1(input)?;
        let (input, keys) = parse_keys(input)?;
        let (input, _) = line_ending(input)?;
        let command: Command = Command::Gets { keys };
        Ok((input, command))
    }
    gets_parser
}
