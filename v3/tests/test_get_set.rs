use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, RwLock};

#[test]
fn test_set_and_get_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));

    let mut output = Vec::new();
    let input_set = "set key 0 10 5\r\nvalue\r\n";
    process_input(&app_state_arc, input_set.as_bytes(), &mut output);
    assert_eq!(std::str::from_utf8(&output), Ok("STORED\r\n"));

    let mut output = Vec::new();
    let input_get = "get key\r\n";
    process_input(&app_state_arc, input_get.as_bytes(), &mut output);
    assert_eq!(
        std::str::from_utf8(&output),
        Ok("VALUE key 0 5\r\nvalue\r\nEND\r\n")
    );
}

#[test]
fn test_set_with_noreploy_and_get_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));

    let mut output = Vec::new();
    let input_set = "set key 0 10 5 noreply\r\nvalue\r\n";
    process_input(&app_state_arc, input_set.as_bytes(), &mut output);
    assert_eq!(std::str::from_utf8(&output), Ok(""));

    let mut output = Vec::new();
    let input_get = "get key\r\n";
    process_input(&app_state_arc, input_get.as_bytes(), &mut output);
    assert_eq!(
        std::str::from_utf8(&output),
        Ok("VALUE key 0 5\r\nvalue\r\nEND\r\n")
    );
}

#[test]
fn test_set_and_get_not_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));

    let mut output = Vec::new();
    let input_set = "set key 0 10 5\r\nvalue\r\n";
    process_input(&app_state_arc, input_set.as_bytes(), &mut output);
    assert_eq!(std::str::from_utf8(&output), Ok("STORED\r\n"));

    let mut output = Vec::new();
    let input_get = "get different_key\r\n";
    process_input(&app_state_arc, input_get.as_bytes(), &mut output);
    assert_eq!(std::str::from_utf8(&output), Ok("END\r\n"));
}

#[test]
fn test_set_and_get_multiple_commands_on_one_request_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(RwLock::new(app_state));

    let mut output = Vec::new();
    let input = "set key 0 10 5\r\nvalue\r\nget key\r\n";
    process_input(&app_state_arc, input.as_bytes(), &mut output);
    assert_eq!(
        std::str::from_utf8(&output),
        Ok("STORED\r\nVALUE key 0 5\r\nvalue\r\nEND\r\n")
    );
}
