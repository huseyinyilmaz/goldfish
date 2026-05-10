use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    parser::command::Command,
    state::{Data, State},
};

pub fn handle_set(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Set {
        key,
        flags,
        timeout,
        noreply,
        value,
        value_size: _,
    } = command
    {
        let mut app_state = state.write().unwrap();
        let data = Data {
            data: value,
            timeout,
            flags,
            time: SystemTime::now(),
        };
        app_state.set_key(key, data);

        if !noreply {
            output.extend_from_slice(b"STORED\r\n");
        }
    }
}
