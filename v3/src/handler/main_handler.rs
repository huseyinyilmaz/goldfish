use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State};

use super::{
    add::handle_add, append::handle_append, delete::handle_delete, get::handle_get,
    prepend::handle_prepend, replace::handle_replace, set::handle_set, version::handle_version,
};

pub fn handle_command(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    match command {
        Command::Version => handle_version(output),
        Command::Set { .. } => handle_set(state, command, output),
        Command::Add { .. } => handle_add(state, command, output),
        Command::Replace { .. } => handle_replace(state, command, output),
        Command::Append { .. } => handle_append(state, command, output),
        Command::Prepend { .. } => handle_prepend(state, command, output),
        Command::Get { .. } => handle_get(state, command, output),
        Command::Delete { .. } => handle_delete(state, command, output),
        _ => output.extend_from_slice(b"ERROR\r\n"),
    }
}
