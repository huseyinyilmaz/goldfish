use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State};

pub fn handle_flush_all(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::FlushAll { noreply, .. } = command {
        let mut app_state = state.write().unwrap();
        app_state.clear();
        if !noreply {
            output.extend_from_slice(b"OK\r\n");
        }
    }
}
