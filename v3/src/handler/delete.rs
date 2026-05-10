use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State};

pub fn handle_delete(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Delete { key, noreply } = command {
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
