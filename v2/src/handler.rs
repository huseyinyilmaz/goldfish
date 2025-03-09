use async_std::io::BufReader;
use async_std::io::BufWriter;
use async_std::io::Read;
use async_std::io::Write;
use async_std::net::TcpStream;
use async_std::prelude::*;
use std::error::Error;

use crate::command::Command;
use crate::parser::Parser;

#[derive(Debug)]
pub struct Handler {
    // buffer: [u8; 1024],
    current: String,
    // reader: R,
    // writer: W,
    stream: TcpStream,
}

impl Handler {
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // while let val = self.stream.get() {
        // }

        // println!("val = {}", val);
        // let bytes_written = self.stream.write(b"XXXXXXXXX Here is what we want to write\n\r").await?;
        // self.stream.flush().await?;
        // println!("bytes-written = {}", bytes_written);
        // let val = self.read_line().await;
        // println!("val = {}", val);

        let mut continue_loop: bool = true;
        let mut buf_reader = BufReader::new(&self.stream);
        let mut buf_writer = BufWriter::new(&self.stream);
        let mut buf: Vec<u8> = Vec::new();
        buf_reader.read_until(b'\n', &mut buf);
        while continue_loop {
            // let mut buf = Vec::new();

            let command = Parser::parse_command(&mut buf_reader).await;
            println!("command: {:?}", command);
            let result: &[u8] = match command {
                Some(Command::Version) => b"VERSION 1.0",
                Some(Command::Quit) => {
                    continue_loop = false;
                    b""
                }
                None => b"CLIENT_ERROR could not recognize command",
                _ => b"SERVER_ERROR not implemented",
            };
            let bytes_written = &buf_writer.write(&result).await?;
            println!("bytes-written = {}", bytes_written);
            //let bytes_written = &buf_writer.write(&[b'\r', b'\n']).await?;
            let bytes_written = &buf_writer.write(b"\r\n").await?;
            println!("bytes-written = {}", bytes_written);
            // let bytes_written = (&self.stream).write(b"XXXXXXXXX Here is what we want to write\n\r").await?;
            // println!("bytes-written = {}", bytes_written);
            buf_writer.flush().await?;
            // (&mut buf_writer).flush();
            // (&self.stream).flush();
            // &buf_writer.flush();
            // let buf_size = buf_reader.read_until(b'\n', &mut buf).await?;
            // println!("Line: {:?}, buf_size{:?}", String::from_utf8(buf), buf_size);
            // if buf_size == 0 {
            //     println!("Stream is closed. Quiting connection.");
            //     continue_loop = false;
            // }
        }
        Ok(())
    }

    pub async fn new(stream: TcpStream) -> Self {
        let mut handler = Handler {
            // buffer: [0, 1024],
            current: String::from(""),
            stream: stream,
        };
        handler.run().await.unwrap();
        handler
    }
}
