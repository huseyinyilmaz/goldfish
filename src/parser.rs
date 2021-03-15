use async_std::stream::Stream;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::io::BufRead;
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
