use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{parser::command::Command, state::State, utils};

use super::set::normalize_timeout;

pub fn handle_touch(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Touch {
        key,
        timeout,
        noreply,
    } = command
    {
        if key.len() > 250 || utils::has_control_chars(&key) {
            output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
            return;
        }
        let mut app_state = state.write().unwrap();
        if let Some(existing) = app_state.get_key(&key) {
            let data = crate::state::Data {
                data: existing.data.clone(),
                timeout: normalize_timeout(timeout),
                flags: existing.flags,
                time: SystemTime::now(),
                cas_unique: 0,
            };
            app_state.set_key(key, data);
            if !noreply {
                output.extend_from_slice(b"TOUCHED\r\n");
            }
        } else if !noreply {
            output.extend_from_slice(b"NOT_FOUND\r\n");
        }
    }
}
