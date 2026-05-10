use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{parser::command::Command, state::State};

fn append_value(output: &mut Vec<u8>, key: &[u8], flags: i32, value: &[u8]) {
    output.extend_from_slice(b"VALUE ");
    output.extend_from_slice(key);
    output.extend_from_slice(b" ");
    output.extend_from_slice(flags.to_string().as_bytes());
    output.extend_from_slice(b" ");
    output.extend_from_slice(value.len().to_string().as_bytes());
    output.extend_from_slice(b"\r\n");
    output.extend_from_slice(value);
    output.extend_from_slice(b"\r\n");
}

pub fn handle_get(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Get { keys } = command {
        let app_state = state.read().unwrap();
        let now = SystemTime::now();
        for key in &keys {
            if let Some(data) = app_state.get_key(key) {
                let duration_since_seconds =
                    now.duration_since(data.time).unwrap_or_default().as_secs();
                if data.timeout == 0 || (data.timeout > 0 && duration_since_seconds < data.timeout as u64) {
                    append_value(output, key, data.flags, &data.data);
                }
            }
        }
        output.extend_from_slice(b"END\r\n");
    }
}
