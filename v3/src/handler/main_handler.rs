use std::sync::{Arc, Mutex};

use crate::{parser::command::Command, state::State};

use super::{get::handle_get, set::handle_set, version::handle_version};

pub fn handle_command(state: &Arc<Mutex<State>>, command: Command, output: &mut Vec<u8>) {
    match command {
        Command::Version => handle_version(output),
        Command::Set { .. } => handle_set(state, command, output),
        Command::Get { .. } => handle_get(state, command, output),
        _ => output.extend_from_slice(b"ERROR\r\n"),
    }
}
