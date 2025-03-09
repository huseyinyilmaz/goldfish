use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    parser::{command::Command, response::CommandResponse},
    state::State,
};

pub fn handle_get(state: &Arc<Mutex<State>>, command: Command) -> CommandResponse {
    if let Command::Get { key } = command {
        let app_state = state.lock().unwrap();

        return match app_state.get_key(&key) {
            Some(data) => {
                let duration_since_seconds = SystemTime::now()
                    .duration_since(data.time)
                    .unwrap_or_default()
                    .as_secs();
                if data.timeout > 0 && duration_since_seconds >= data.timeout {
                    return CommandResponse::GetNotFound;
                } else {
                    return CommandResponse::Get {
                        key,
                        flags: data.flags,
                        data: data.data.clone(),
                    };
                }
            }
            None => CommandResponse::GetNotFound,
        };
    } else {
        return CommandResponse::Error;
    }
}
