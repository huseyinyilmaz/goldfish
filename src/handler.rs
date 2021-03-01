use std::net::TcpStream;
use std::io::Write;
use std::error::Error;

#[derive(Debug)]
pub struct Handler {
    stream: TcpStream
}

impl Handler {

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let bytes_written = self.stream.write(b"Here is what we want to write\n\r")?;
        println!("bytes-written = {}", bytes_written);
        Ok(())
    }

    pub fn new(stream: TcpStream) -> Self {
        let mut handler = Handler {stream: stream};
        handler.run().unwrap();
        handler
    }
}
