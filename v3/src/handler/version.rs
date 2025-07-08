use std::sync::{Arc, Mutex};

use crate::{
    parser::{command::Command, response::CommandResponse},
    state::State,
};

pub fn handle_version(_state: &Arc<Mutex<State>>, _command: Command) -> CommandResponse {
    return CommandResponse::Version(String::from("Goldfish 1.0"));
}
