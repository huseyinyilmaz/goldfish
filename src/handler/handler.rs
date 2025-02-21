use std::sync::{Arc, Mutex};

use crate::{
    parser::{command::Command, response::CommandResponse},
    state::State,
};

use super::{get::handle_get, set::handle_set, version::handle_version};

pub fn handle_command(state: &Arc<Mutex<State>>, command: Command) -> CommandResponse {
    match command {
        Command::Version => handle_version(state, command),
        Command::Set { .. } => handle_set(state, command),
        Command::Get { .. } => handle_get(state, command),
        _ => CommandResponse::Error,
    }
}
