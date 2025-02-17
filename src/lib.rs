mod shutdown;
mod parser;
mod utils;
mod settings;

use std::net::SocketAddr;
use std::time::Duration;

use log::info;
use nom::Parser;
use parser::command::Command;
use parser::make_parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpListener, time::sleep};

// use tokio::net::TcpListener;
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Goldfish Server.");
    sleep(Duration::from_secs(1)).await;
    // let shutdown_handler = shutdown::ShutdownHandler::new();
    let settings = settings::Settings::new()?;
    // Combine the IP address and port into a SocketAddr
    let socket_addr = SocketAddr::new(settings.ip_address, settings.port);

    info!("{:?}", &settings);
    // Bind the listener to the address
    let listener = TcpListener::bind(socket_addr).await?;
    info!("Listening on {}...", socket_addr);
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                info!("Number of bytes read: {}", n);
                info!("{:?}", utils::raw_string_to_string(&buf));
                let mut parser = make_parser();
                let (_rest_buf, command) = parser.parse(&buf).unwrap();
                info!("Command = {:?}", command);
                // If command is quit just break the loop that waits for the socket data.
                // This will close the connection with the client.
                if command == Command::Quit {
                    info!("Quit Command Received");
                    break;
                }

                // if is_shutdown_triggered.load(Ordering::SeqCst) {
                //     info!("Ctrl+C received, shutting down...");
                //     process::exit(0);
                // }
                
                // if n == 0 {
                //     return;
                // }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });

    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_server().await
}
