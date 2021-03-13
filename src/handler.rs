use async_std::prelude::*;
use async_std::net::TcpStream;
// use async_std::io::Write;
use async_std::io::BufReader;
use std::error::Error;

#[derive(Debug)]
pub struct Handler {
    // buffer: [u8; 1024],
    current: String,
    stream: TcpStream,
}

impl Handler {

    async fn read_more(&mut self) -> Result<(), Box<dyn Error>> {
        let chars_read = self.stream.read_to_string(&mut self.current).await?;
        println!("chars_read = {}, result: {}", &chars_read, &self.current);
        Ok(())
    }


    async fn read_line(&mut self) -> String {
        self.read_more().await.unwrap();
        println!("chars_read = {}", self.current);
        String::from("")
    }

    // async fn read_next(&mut self) -> String {
    //     self.stream.read_until
    // }

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // while let val = self.stream.get() {
        // }

        // println!("val = {}", val);
        let bytes_written = self.stream.write(b"XXXXXXXXX Here is what we want to write\n\r").await?;
        self.stream.flush().await?;
        println!("bytes-written = {}", bytes_written);
        // let val = self.read_line().await;
        // println!("val = {}", val);

        let mut continue_loop: bool = true;
        let mut buf_reader = BufReader::new(&mut self.stream);
        while continue_loop {
            let mut buf = Vec::new();
            println!("read_until");
            let buf_size = buf_reader.read_until(b'\n', &mut buf).await?;
            println!("Line: {:?}, buf_size{:?}", String::from_utf8(buf), buf_size);
            if buf_size == 0 {
                println!("Stream is closed. Quiting connection.");
                continue_loop = false;
            }
        }
        Ok(())
    }

    pub async fn new(stream: TcpStream) -> Self {
        let mut handler = Handler {
            // buffer: [0, 1024],
            current: String::from(""),
            stream: stream};
        handler.run().await.unwrap();
        handler
    }
}
