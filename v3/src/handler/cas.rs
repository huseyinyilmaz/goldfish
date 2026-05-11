use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    parser::command::Command,
    state::{Data, State},
    utils,
};

pub fn handle_cas(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Cas {
        key,
        flags,
        timeout,
        noreply,
        value,
        value_size: _,
        cas_unique,
    } = command
    {
        if key.len() > 250 || utils::has_control_chars(&key) {
            output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
            return;
        }
        let mut app_state = state.write().unwrap();
        if let Some(existing) = app_state.get_key(&key) {
            if existing.cas_unique != cas_unique {
                if !noreply {
                    output.extend_from_slice(b"EXISTS\r\n");
                }
            } else {
                let data = Data {
                    data: value,
                    timeout: crate::handler::set::normalize_timeout(timeout),
                    flags,
                    time: SystemTime::now(),
                    cas_unique: 0,
                };
                app_state.set_key(key, data);
                app_state.increment_total_items();
                if !noreply {
                    output.extend_from_slice(b"STORED\r\n");
                }
            }
        } else if !noreply {
            output.extend_from_slice(b"NOT_FOUND\r\n");
        }
    }
}
