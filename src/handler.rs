use async_std::prelude::*;
use async_std::net::TcpStream;
// use async_std::io::Write;
use std::error::Error;

#[derive(Debug)]
pub struct Handler {
    stream: TcpStream
}

impl Handler {

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let bytes_written = self.stream.write(b"XXXXXXXXX Here is what we want to write\n\r").await?;
        self.stream.flush().await?;
        println!("bytes-written = {}", bytes_written);
        Ok(())
    }

    pub async fn new(stream: TcpStream) -> Self {
        let mut handler = Handler {stream: stream};
        handler.run().await.unwrap();
        handler
    }
}
