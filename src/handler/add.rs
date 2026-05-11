use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    handler::set::normalize_timeout,
    parser::command::Command,
    state::{Data, State},
    utils,
};

pub fn handle_add(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Add {
        key,
        flags,
        timeout,
        noreply,
        value,
        value_size: _,
    } = command
    {
        if key.len() > 250 || utils::has_control_chars(&key) {
            output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
            return;
        }
        let mut app_state = state.write().unwrap();
        if app_state.get_key(&key).is_some() {
            if !noreply {
                output.extend_from_slice(b"NOT_STORED\r\n");
            }
        } else {
            let data = Data {
                data: value,
                timeout: normalize_timeout(timeout),
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
    }
}
