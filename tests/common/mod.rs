use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, RwLock};

pub fn new_state() -> Arc<RwLock<State>> {
    Arc::new(RwLock::new(State::new()))
}

pub fn process(state: &Arc<RwLock<State>>, input: &str) -> String {
    let mut output = Vec::new();
    process_input(state, input.as_bytes(), &mut output);
    String::from_utf8(output).unwrap()
}

#[allow(dead_code)]
pub fn process_raw(state: &Arc<RwLock<State>>, input: &[u8]) -> (bool, Vec<u8>) {
    let mut output = Vec::new();
    let ok = process_input(state, input, &mut output);
    (ok, output)
}
