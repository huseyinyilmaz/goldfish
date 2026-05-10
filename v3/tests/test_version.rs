use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, Mutex};

#[test]
fn test_version() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));
    let mut output = Vec::new();
    let input = "version\r\n";
    process_input(&app_state_arc, input.as_bytes(), &mut output);
    assert_eq!(std::str::from_utf8(&output), Ok("VERSION Goldfish 1.0\r\n"));
}
