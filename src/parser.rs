use async_std::io::BufRead;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::stream::Stream;
// use async_std::io::BufReadExt;
// use async_std::io::BufReadExt;

use crate::command::Command;
use async_std::prelude::*;

pub struct Parser;

impl Parser {
    pub async fn parse_command<R: BufRead + Unpin>(reader: &mut R) -> Option<Command> {
        let mut buf = Vec::new();
        let buf_size = reader.read_until(b'\n', &mut buf).await;
        println!("Reading: {:?}", buf);
        let version_text: Vec<u8> = b"version\r\n".to_vec();
        let quit_text: Vec<u8> = b"quit\r\n".to_vec();
        match buf {
            txt if txt == version_text => Some(Command::Version),
            txt if txt == quit_text => Some(Command::Quit),
            _ => None,
        }
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
