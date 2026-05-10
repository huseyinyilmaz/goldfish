use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State};

pub fn handle_stats(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    if let Command::Stats { sub } = command {
        if sub.is_some() {
            output.extend_from_slice(b"END\r\n");
            return;
        }
        let app_state = state.read().unwrap();
        let now = std::time::SystemTime::now();
        let uptime = now
            .duration_since(app_state.start_time)
            .unwrap_or_default()
            .as_secs();
        let time_secs = now
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let curr_items = app_state.len();
        let total_items = app_state
            .total_items
            .load(std::sync::atomic::Ordering::Relaxed);
        let bytes = app_state.total_bytes();
        let cmd_get = app_state.cmd_get.load(std::sync::atomic::Ordering::Relaxed);
        let cmd_set = app_state.cmd_set.load(std::sync::atomic::Ordering::Relaxed);
        let get_hits = app_state
            .get_hits
            .load(std::sync::atomic::Ordering::Relaxed);
        let get_misses = app_state
            .get_misses
            .load(std::sync::atomic::Ordering::Relaxed);
        let pointer_size = (std::mem::size_of::<usize>() * 8) as u64;

        write_stat(output, "pid", &std::process::id().to_string());
        write_stat(output, "uptime", &uptime.to_string());
        write_stat(output, "time", &time_secs.to_string());
        write_stat(output, "version", "goldfish 0.1.0");
        write_stat(output, "pointer_size", &pointer_size.to_string());
        write_stat(output, "curr_items", &curr_items.to_string());
        write_stat(output, "total_items", &total_items.to_string());
        write_stat(output, "bytes", &bytes.to_string());
        write_stat(output, "curr_connections", "0");
        write_stat(output, "total_connections", "0");
        write_stat(output, "connection_structures", "0");
        write_stat(output, "cmd_get", &cmd_get.to_string());
        write_stat(output, "cmd_set", &cmd_set.to_string());
        write_stat(output, "get_hits", &get_hits.to_string());
        write_stat(output, "get_misses", &get_misses.to_string());
        write_stat(output, "evictions", "0");
        write_stat(output, "bytes_read", "0");
        write_stat(output, "bytes_written", "0");
        write_stat(output, "limit_maxbytes", "0");
        output.extend_from_slice(b"END\r\n");
    }
}

fn write_stat(output: &mut Vec<u8>, key: &str, value: &str) {
    output.extend_from_slice(b"STAT ");
    output.extend_from_slice(key.as_bytes());
    output.extend_from_slice(b" ");
    output.extend_from_slice(value.as_bytes());
    output.extend_from_slice(b"\r\n");
}
