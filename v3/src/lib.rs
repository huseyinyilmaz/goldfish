mod handler;
mod parser;
mod settings;
pub mod state;
mod utils;

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use log::{debug, info};
use nom::Parser;
use parser::command::Command;
use parser::make_parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpListener, time::sleep};

pub fn process_input(
    state: &Arc<RwLock<state::State>>,
    input: &[u8],
    output: &mut Vec<u8>,
) -> bool {
    let mut parser = make_parser();
    let mut parser_input = input;
    while !parser_input.is_empty() {
        let (rest, command) = parser.parse(parser_input).unwrap();
        parser_input = rest;
        if command == Command::Quit {
            return false;
        }
        handler::handle_command(state, command, output);
    }
    true
}

async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Goldfish Server.");
    sleep(Duration::from_secs(1)).await;
    let app_settings = settings::Settings::new()?;
    let app_state = state::State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));
    let socket_addr = SocketAddr::new(app_settings.ip_address, app_settings.port);
    let listener = TcpListener::bind(socket_addr).await?;
    info!("Listening on {socket_addr}");
    loop {
        let app_state_arc_clone = app_state_arc.clone();
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024 * 1024 * 10];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");

                debug!("Number of bytes read = {n}");
                debug!("Raw Request bytestring = {:?}", &buf[..n]);
                debug!("Raw Request= {:?}", utils::raw_string_to_string(&buf[..n]));

                if n > 0 {
                    let mut output = Vec::new();
                    if !process_input(&app_state_arc_clone, &buf[..n], &mut output) {
                        debug!("Quit Command Received. Closing TCP connection.");
                        break;
                    }
                    debug!("Raw Response = {:?}", utils::raw_string_to_string(&output));
                    debug!("State = {app_state_arc_clone:?}");
                    socket
                        .write_all(&output)
                        .await
                        .expect("failed to write data to socket");
                } else {
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
