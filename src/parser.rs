use async_std::io::BufRead;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::stream::Stream;
// use async_std::io::BufReadExt;
// use async_std::io::BufReadExt;
use nom::{
    branch::alt,
    bytes::complete,
    eof,
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    sequence::tuple,
    // combinator::alt,
    IResult,
};

use crate::command::Command;
use async_std::prelude::*;

pub fn parse_version(input: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, _)) =
        tuple((complete::tag(&b"version"[..]), complete::tag(&b"\r\n"[..])))(input)?;
    Ok((input, Command::Version))
}

pub fn parse_quit(input: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, _)) = tuple((complete::tag(&b"quit"[..]), complete::tag(&b"\r\n"[..])))(input)?;
    Ok((input, Command::Quit))
}

pub fn parse(input: &[u8]) -> IResult<&[u8], Command> {
    let mut parser = alt((parse_version, parse_quit));
    parser(input)
}

pub struct Parser;

impl Parser {
    pub async fn parse_command<R: BufRead + Unpin>(reader: &mut R) -> Option<Command> {
        let mut buf = Vec::new();
        let buf_size = reader.read_until(b'\n', &mut buf).await;
        println!("Reading: {:?}", buf);
        let version_text: Vec<u8> = b"version\r\n".to_vec();
        let quit_text: Vec<u8> = b"quit\r\n".to_vec();
        match parse(&buf) {
            Ok((_buf, command)) => Some(command),
            _ => None,
        }
        // let Ok((buf, command)) = parse(&buf);
        // Some(command)
        // match buf {
        //     txt if txt == version_text => Some(Command::Version),
        //     txt if txt == quit_text => Some(Command::Quit),
        //     _ => None,
        // }
    }

    pub fn new() -> Self {
        Parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn parse_quit() {
        let mut input: &[u8] = b"quit\r\n";
        let result = Parser::parse_command(&mut input).await;
        let expected = Some(Command::Quit);
        assert_eq!(result, expected);
    }

    #[async_std::test]
    async fn parse_version() {
        let mut input: &[u8] = b"version\r\n";
        let result = Parser::parse_command(&mut input).await;
        let expected = Some(Command::Version);
        assert_eq!(result, expected);
    }

    #[async_std::test]
    async fn parse_quit_and_version() {
        let mut input: &[u8] = b"quit\r\nversion\r\n";
        let mut result = Parser::parse_command(&mut input).await;
        let mut expected = Some(Command::Quit);
        assert_eq!(result, expected);
        result = Parser::parse_command(&mut input).await;
        expected = Some(Command::Version);
        assert_eq!(result, expected);
    }
}
