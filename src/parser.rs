use async_std::stream::Stream;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use crate::command::Command;

pub struct Parser;

impl Parser {
    pub async fn parse_command(reader: BufReader<TcpStream>) -> Option<Command> {

        Some(Command::Version)
    }


    pub fn new() -> Self {
        Parser
    }
}
