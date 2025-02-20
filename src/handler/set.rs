use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    parser::{command::Command, response::CommandResponse},
    state::{Data, State},
};

pub fn handle_set(state: &Arc<Mutex<State>>, command: Command) -> CommandResponse {
    if let Command::Set {
        key,
        flags,
        timeout,
        noreply,
        value,
        value_size: _,
    } = command
    {
        let mut app_state = state.lock().unwrap();
        let time = SystemTime::now();
        let data = Data {
            data: value,
            timeout,
            flags,
            time,
        };
        app_state.set_key(key, data);

        if noreply {
            return CommandResponse::Set(String::from(""));
        } else {
            return CommandResponse::Set(String::from("STORED\r\n"));
        }
    } else {
        return CommandResponse::Error;
    }
}
