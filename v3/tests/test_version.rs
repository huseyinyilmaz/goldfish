use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, Mutex};

#[test]
fn test_version() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));
    let input = "version\r\n";
    let expected_output = "VERSION Goldfish 1.0\r\n";
    let result = process_input(&app_state_arc, input.as_bytes());
    assert_eq!(std::str::from_utf8(&result.unwrap()), Ok(expected_output));
}
