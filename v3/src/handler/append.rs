use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    parser::command::Command,
    state::{Data, State},
};

pub fn handle_append(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Append {
        key,
        value,
        noreply,
        ..
    } = command
    {
        if key.len() > 250 {
            output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
            return;
        }
        let mut app_state = state.write().unwrap();
        if let Some(existing) = app_state.get_key(&key) {
            let mut new_data = existing.data.clone();
            new_data.extend_from_slice(&value);
            let data = Data {
                data: new_data,
                timeout: existing.timeout,
                flags: existing.flags,
                time: SystemTime::now(),
            };
            app_state.set_key(key, data);
            if !noreply {
                output.extend_from_slice(b"STORED\r\n");
            }
        } else if !noreply {
            output.extend_from_slice(b"NOT_STORED\r\n");
        }
    }
}
