use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State, utils};

pub fn handle_delete(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Delete { key, noreply } = command {
        if key.len() > 250 || utils::has_control_chars(&key) {
            output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
            return;
        }
        let mut app_state = state.write().unwrap();
        if app_state.delete_key(&key) {
            if !noreply {
                output.extend_from_slice(b"DELETED\r\n");
            }
        } else if !noreply {
            output.extend_from_slice(b"NOT_FOUND\r\n");
        }
    }
}
