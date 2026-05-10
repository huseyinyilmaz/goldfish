use std::{
    str::FromStr,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    parser::command::Command,
    state::{Data, State},
};

pub fn handle_incr(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Incr {
        key,
        delta,
        noreply,
    } = command
    {
        let mut app_state = state.write().unwrap();
        if let Some(existing) = app_state.get_key(&key) {
            let value_str = std::str::from_utf8(&existing.data).unwrap_or("");
            match u64::from_str(value_str) {
                Ok(current) => {
                    let new_value = current.wrapping_add(delta);
                    let new_data = new_value.to_string().into_bytes();
                    let data = Data {
                        data: new_data,
                        timeout: existing.timeout,
                        flags: existing.flags,
                        time: SystemTime::now(),
                    };
                    app_state.set_key(key, data);
                    if !noreply {
                        output.extend_from_slice(new_value.to_string().as_bytes());
                        output.extend_from_slice(b"\r\n");
                    }
                }
                Err(_) => {
                    output.extend_from_slice(
                        b"CLIENT_ERROR cannot increment or decrement non-numeric value\r\n",
                    );
                }
            }
        } else if !noreply {
            output.extend_from_slice(b"NOT_FOUND\r\n");
        }
    }
}
