use std::sync::{Arc, Mutex};

use crate::{
    parser::{command::Command, response::CommandResponse},
    state::State,
};

use super::version::handle_version;

pub fn handle_command(state: &Arc<Mutex<State>>, command: &Command) -> CommandResponse {
    match command {
        Command::Version => handle_version(state, command),
        _ => CommandResponse::UnhandledCommand(String::from("Cannot handle command")),
    }
}
