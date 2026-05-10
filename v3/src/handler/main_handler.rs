use std::sync::{Arc, RwLock};

use crate::{parser::command::Command, state::State};

use std::sync::atomic::Ordering;

use super::{
    add::handle_add, append::handle_append, decr::handle_decr, delete::handle_delete,
    flush_all::handle_flush_all, get::handle_get, incr::handle_incr, prepend::handle_prepend,
    replace::handle_replace, set::handle_set, stats::handle_stats, version::handle_version,
};

pub fn handle_command(state: &Arc<RwLock<State>>, command: Command, output: &mut Vec<u8>) {
    match command {
        Command::Version => handle_version(output),
        Command::Set { .. } => {
            state
                .read()
                .unwrap()
                .cmd_set
                .fetch_add(1, Ordering::Relaxed);
            handle_set(state, command, output);
        }
        Command::Add { .. } => {
            state
                .read()
                .unwrap()
                .cmd_set
                .fetch_add(1, Ordering::Relaxed);
            handle_add(state, command, output);
        }
        Command::Replace { .. } => {
            state
                .read()
                .unwrap()
                .cmd_set
                .fetch_add(1, Ordering::Relaxed);
            handle_replace(state, command, output);
        }
        Command::Append { .. } => {
            state
                .read()
                .unwrap()
                .cmd_set
                .fetch_add(1, Ordering::Relaxed);
            handle_append(state, command, output);
        }
        Command::Prepend { .. } => {
            state
                .read()
                .unwrap()
                .cmd_set
                .fetch_add(1, Ordering::Relaxed);
            handle_prepend(state, command, output);
        }
        Command::Incr { .. } => {
            handle_incr(state, command, output);
        }
        Command::Decr { .. } => {
            handle_decr(state, command, output);
        }
        Command::Get { .. } => {
            state
                .read()
                .unwrap()
                .cmd_get
                .fetch_add(1, Ordering::Relaxed);
            handle_get(state, command, output);
        }
        Command::FlushAll { .. } => {
            handle_flush_all(state, command, output);
        }
        Command::Delete { .. } => {
            handle_delete(state, command, output);
        }
        Command::Stats { .. } => {
            handle_stats(state, command, output);
        }
        Command::Malformed => output.extend_from_slice(b"CLIENT_ERROR bad command line format\r\n"),
        _ => output.extend_from_slice(b"ERROR\r\n"),
    }
}
