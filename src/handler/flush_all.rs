use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::{parser::command::Command, state::State};

pub fn handle_flush_all(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::FlushAll { delay, noreply } = command {
        if delay > 0 {
            let state = state.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(delay));
                state.write().unwrap().clear();
            });
        } else {
            state.write().unwrap().clear();
        }
        if !noreply {
            output.extend_from_slice(b"OK\r\n");
        }
    }
}
