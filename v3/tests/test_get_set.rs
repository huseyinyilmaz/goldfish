use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, Mutex};

#[test]
fn test_set_and_get_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let input_set = "set key 0 10 5\r\nvalue\r\n";
    let expected_output_set = "STORED\r\n";

    let result_set = process_input(&app_state_arc, input_set.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_set.unwrap()),
        Ok(expected_output_set)
    );

    let input_get = "get key\r\n";
    let expected_output_get = "VALUE key 0 5\r\nvalue\r\nEND\r\n";
    let result_get = process_input(&app_state_arc, input_get.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_get.unwrap()),
        Ok(expected_output_get)
    );
}

#[test]
fn test_set_with_noreploy_and_get_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let input_set = "set key 0 10 5 noreply\r\nvalue\r\n";
    let expected_output_set = "";

    let result_set = process_input(&app_state_arc, input_set.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_set.unwrap()),
        Ok(expected_output_set)
    );

    let input_get = "get key\r\n";
    let expected_output_get = "VALUE key 0 5\r\nvalue\r\nEND\r\n";
    let result_get = process_input(&app_state_arc, input_get.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_get.unwrap()),
        Ok(expected_output_get)
    );
}

#[test]
fn test_set_and_get_not_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let input_set = "set key 0 10 5\r\nvalue\r\n";
    let expected_output_set = "STORED\r\n";

    let result_set = process_input(&app_state_arc, input_set.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_set.unwrap()),
        Ok(expected_output_set)
    );

    let input_get = "get different_key\r\n";
    let expected_output_get = "END\r\n";
    let result_get = process_input(&app_state_arc, input_get.as_bytes());
    assert_eq!(
        std::str::from_utf8(&result_get.unwrap()),
        Ok(expected_output_get)
    );
}

#[test]
fn test_set_and_get_multiple_commands_on_one_request_found() {
    let app_state = State::new();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let input = "set key 0 10 5\r\nvalue\r\nget key\r\n";
    let expected_output = "STORED\r\nVALUE key 0 5\r\nvalue\r\nEND\r\n";

    let result = process_input(&app_state_arc, input.as_bytes());
    assert_eq!(std::str::from_utf8(&result.unwrap()), Ok(expected_output));
}
