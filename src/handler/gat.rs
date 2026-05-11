use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{parser::command::Command, state::State, utils};

use super::set::normalize_timeout;

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

fn append_value_with_cas(
    output: &mut Vec<u8>,
    key: &[u8],
    flags: i32,
    value: &[u8],
    cas_unique: u64,
) {
    output.extend_from_slice(b"VALUE ");
    output.extend_from_slice(key);
    output.extend_from_slice(b" ");
    output.extend_from_slice(flags.to_string().as_bytes());
    output.extend_from_slice(b" ");
    output.extend_from_slice(value.len().to_string().as_bytes());
    output.extend_from_slice(b" ");
    output.extend_from_slice(cas_unique.to_string().as_bytes());
    output.extend_from_slice(b"\r\n");
    output.extend_from_slice(value);
    output.extend_from_slice(b"\r\n");
}

fn handle_gat_or_gats(
    state: &Arc<RwLock<State>>,
    command: Command,
    output: &mut Vec<u8>,
    with_cas: bool,
) {
    let (timeout, keys) = match command {
        Command::Gat { timeout, keys } => (timeout, keys),
        Command::Gats { timeout, keys } => (timeout, keys),
        _ => return,
    };

    if keys
        .iter()
        .any(|k| k.len() > 250 || utils::has_control_chars(k))
    {
        output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n");
        return;
    }

    let mut app_state = state.write().unwrap();
    let now = SystemTime::now();
    let normalized_timeout = normalize_timeout(timeout);
    let mut found = false;

    for key in &keys {
        if let Some(existing) = app_state.get_key(key) {
            let duration_since_seconds = now
                .duration_since(existing.time)
                .unwrap_or_default()
                .as_secs();
            if existing.timeout == 0
                || (existing.timeout > 0 && duration_since_seconds < existing.timeout as u64)
            {
                if with_cas {
                    append_value_with_cas(
                        output,
                        key,
                        existing.flags,
                        &existing.data,
                        existing.cas_unique,
                    );
                } else {
                    append_value(output, key, existing.flags, &existing.data);
                }
                found = true;

                let data = crate::state::Data {
                    data: existing.data.clone(),
                    timeout: normalized_timeout,
                    flags: existing.flags,
                    time: SystemTime::now(),
                    cas_unique: 0,
                };
                app_state.set_key(key.clone(), data);
            }
        }
    }

    if found {
        app_state
            .get_hits
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    } else {
        app_state
            .get_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    output.extend_from_slice(b"END\r\n");
}

pub fn handle_gat(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    handle_gat_or_gats(state, command, output, false);
}

pub fn handle_gats(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    handle_gat_or_gats(state, command, output, true);
}
