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

pub fn handle_replace(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Replace {
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
        if app_state.get_key(&key).is_none() {
            if !noreply {
                output.extend_from_slice(b"NOT_STORED\r\n");
            }
        } else {
            let data = Data {
                data: value,
                timeout: normalize_timeout(timeout),
                flags,
                time: SystemTime::now(),
            };
            app_state.set_key(key, data);
            if !noreply {
                output.extend_from_slice(b"STORED\r\n");
            }
        }
    }
}
