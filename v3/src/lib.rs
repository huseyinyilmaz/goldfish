mod handler;
mod parser;
mod settings;
pub mod state;
mod utils;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::{debug, info};
use nom::Parser;
use parser::command::Command;
use parser::make_parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpListener, time::sleep};

pub fn process_input(state: &Arc<Mutex<state::State>>, input: &[u8]) -> Option<Vec<u8>> {
    let mut parser = make_parser();
    // Parser has a generic catch all parser at the end.
    // It will always return a value. That is why unwrap will not panic
    let mut result: Vec<u8> = Vec::new();
    let mut parser_input = input;
    while !parser_input.is_empty() {
        let (rest, command) = parser.parse(parser_input).unwrap();
        parser_input = rest;
        // If command is quit just break the loop that waits for the socket data.
        // This will close the connection with the client.
        if command == Command::Quit {
            return None;
        } else {
            let response_command = handler::handle_command(&state, command);
            debug!("Response = {:?}", response_command);
            let response_command_vec = response_command.as_vec();
            if result.is_empty() {
                result = response_command_vec;
            } else {
                result.extend(response_command_vec);
            }
        }
    }
    return Some(result);
}

// use tokio::net::TcpListener;
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Goldfish Server.");
    sleep(Duration::from_secs(1)).await;
    let app_settings = settings::Settings::new()?;
    let app_state = state::State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));
    // Combine the IP address and port into a SocketAddr
    let socket_addr = SocketAddr::new(app_settings.ip_address, app_settings.port);
    // Bind the listener to the address
    let listener = TcpListener::bind(socket_addr).await?;
    info!("Listening on {}", socket_addr);
    loop {
        let app_state_arc_clone = app_state_arc.clone();
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024 * 1024 * 10]; //10 MB buffer to read
                                                     // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");

                debug!("Number of bytes read = {}", n);
                debug!("Raw Request bytestring = {:?}", &buf[..n]);
                debug!("Raw Request= {:?}", utils::raw_string_to_string(&buf[..n]));

                if n > 0 {
                    match process_input(&app_state_arc_clone, &buf[..n]) {
                        None => {
                            debug!("Quit Command Received. Closing TCP connection.");
                            break;
                        }
                        Some(response) => {
                            debug!(
                                "Raw Response = {:?}",
                                utils::raw_string_to_string(&response)
                            );
                            debug!("State = {:?}", app_state_arc_clone);
                            socket
                                .write_all(&response)
                                .await
                                .expect("failed to write data to socket");
                        }
                    }
                } else {
                    // If n == 0 bytes that means other side closed the connection.
                    // In this case we do not need to listen on our side.
                    debug!("Connection closed.");
                    break;
                }
            }
        });
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_server().await
}
