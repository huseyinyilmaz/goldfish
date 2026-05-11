mod handler;
mod parser;
pub mod settings;
pub mod state;
mod utils;

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use log::{debug, info};
use nom::Parser;
use parser::command::Command;
use parser::main_parser::starts_with_command_keyword;
use parser::make_parser;
use settings::Settings;
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

pub fn process_input_buffered(
    state: &Arc<RwLock<state::State>>,
    buf: &mut Vec<u8>,
    output: &mut Vec<u8>,
) -> bool {
    while !buf.is_empty() {
        let consumed = {
            let mut parser = make_parser();
            let input = buf.as_slice();
            let input_len = input.len();
            match parser.parse(input) {
                Ok((rest, command)) => {
                    let consumed = input_len - rest.len();
                    if command == Command::Quit {
                        return false;
                    }
                    handler::handle_command(state, command, output);
                    Some(consumed)
                }
                Err(nom::Err::Incomplete(_)) => None,
                Err(_) => Some(0),
            }
        };
        match consumed {
            Some(n) => {
                if n > 0 {
                    buf.drain(..n);
                } else if starts_with_command_keyword(buf) && !buf.windows(2).any(|w| w == b"\r\n")
                {
                    return true;
                } else {
                    if let Some(pos) = buf.windows(2).position(|w| w == b"\r\n") {
                        buf.drain(..pos + 2);
                    } else {
                        buf.clear();
                    }
                    output.extend_from_slice(b"ERROR\r\n");
                }
            }
            None => return true,
        }
    }
    true
}

async fn run_server(
    app_settings: Settings,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Goldfish Server.");
    sleep(Duration::from_secs(1)).await;
    let app_state = state::State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));
    let socket_addr = SocketAddr::new(app_settings.ip_address, app_settings.port);
    let listener = TcpListener::bind(socket_addr).await?;
    info!("Listening on {socket_addr}");
    loop {
        let app_state_arc_clone = app_state_arc.clone();
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut read_buf: Vec<u8> = Vec::new();
            let mut tmp = vec![0u8; 65536];

            loop {
                let n = match socket.read(&mut tmp).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("read error: {e}");
                        break;
                    }
                };

                debug!("Number of bytes read = {n}");
                debug!("Raw Request bytestring = {:?}", &tmp[..n]);
                debug!("Raw Request= {:?}", utils::raw_string_to_string(&tmp[..n]));

                read_buf.extend_from_slice(&tmp[..n]);

                let mut output = Vec::new();
                if !process_input_buffered(&app_state_arc_clone, &mut read_buf, &mut output) {
                    debug!("Quit Command Received. Closing TCP connection.");
                    break;
                }
                debug!("Raw Response = {:?}", utils::raw_string_to_string(&output));
                debug!("State = {app_state_arc_clone:?}");

                if !output.is_empty() {
                    if let Err(e) = socket.write_all(&output).await {
                        log::error!("write error: {e}");
                        break;
                    }
                }
            }
        });
    }
}

pub async fn run(app_settings: Settings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_server(app_settings).await
}
