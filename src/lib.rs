mod handler;
mod parser;
mod command;
// use std::io;
// use std::net;
// use std::thread;
use handler::Handler;

use async_std::{
    prelude::*, // 1
    task, // 2
    net::{TcpListener, ToSocketAddrs}, // 3
};



async fn accept_loop(addr: impl ToSocketAddrs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> { // 1

    let listener = TcpListener::bind(addr).await?; // 2
    let mut incoming = listener.incoming();
    while let Some(Ok(stream)) = incoming.next().await { // 3
        let t = task::spawn(Handler::new(stream));
        // TODO manage this task.
        println!("Task started {:?}", &t);
    };
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut)
}
