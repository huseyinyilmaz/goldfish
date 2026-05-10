use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{parser::command::Command, state::State};

pub fn handle_get(state: &Arc<Mutex<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Get { key } = command {
        let app_state = state.lock().unwrap();
        match app_state.get_key(&key) {
            Some(data) => {
                let duration_since_seconds = SystemTime::now()
                    .duration_since(data.time)
                    .unwrap_or_default()
                    .as_secs();
                if data.timeout > 0 && duration_since_seconds >= data.timeout {
                    output.extend_from_slice(b"END\r\n");
                } else {
                    output.extend_from_slice(b"VALUE ");
                    output.extend_from_slice(&key);
                    output.extend_from_slice(b" ");
                    output.extend_from_slice(data.flags.to_string().as_bytes());
                    output.extend_from_slice(b" ");
                    output.extend_from_slice(data.data.len().to_string().as_bytes());
                    output.extend_from_slice(b"\r\n");
                    output.extend_from_slice(&data.data);
                    output.extend_from_slice(b"\r\n");
                    output.extend_from_slice(b"END\r\n");
                }
            }
            None => output.extend_from_slice(b"END\r\n"),
        }
    }
}
