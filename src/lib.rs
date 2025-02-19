mod handler;
mod parser;
mod settings;
mod shutdown;
mod state;
mod utils;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
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
    let app_settings = settings::Settings::new()?;
    let app_state = state::State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));
    // Combine the IP address and port into a SocketAddr
    let socket_addr = SocketAddr::new(app_settings.ip_address, app_settings.port);
    info!("Settings");
    info!("{:?}", &app_settings);
    // Bind the listener to the address
    let listener = TcpListener::bind(socket_addr).await?;
    info!("Listening on {}", socket_addr);
    loop {
        let app_state_arc_clone = app_state_arc.clone();
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");
                if n > 0 {
                    info!("Number of bytes read = {}", n);
                    info!("Raw Request bytestring = {:?}", &buf[..n]);
                    info!("Raw Request= {:?}", utils::raw_string_to_string(&buf[..n]));
                    let mut parser = make_parser();
                    let (_rest_buf, command) = parser.parse(&buf).unwrap();
                    info!("Command = {:?}", command);
                    // If command is quit just break the loop that waits for the socket data.
                    // This will close the connection with the client.
                    if command == Command::Quit {
                        info!("Quit Command Received. Closing TCP connection.");
                        break;
                    }

                    let response = handler::handle_command(&app_state_arc_clone, &command);

                    socket
                        .write_all(response.as_vec().as_slice())
                        .await
                        .expect("failed to write data to socket");
                }
            }
        });
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_server().await
}
