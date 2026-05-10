use goldfish::process_input;

mod common;

#[test]
fn test_quit_basic() {
    let state = common::new_state();
    let mut output = Vec::new();
    let result = process_input(&state, b"quit\r\n", &mut output);
    assert!(!result);
    assert!(output.is_empty());
}

#[test]
fn test_quit_pipeline_termination() {
    let state = common::new_state();
    let mut output = Vec::new();
    let result = process_input(
        &state,
        b"version\r\nquit\r\nversion\r\n",
        &mut output,
    );
    assert!(!result);
    assert_eq!(output, b"VERSION Goldfish 1.0\r\n");
}

#[test]
fn test_quit_after_set() {
    let state = common::new_state();
    let mut output = Vec::new();
    let result = process_input(
        &state,
        b"set key 0 0 5\r\nhello\r\nquit\r\n",
        &mut output,
    );
    assert!(!result);
    assert_eq!(output, b"STORED\r\n");
}

#[test]
fn test_quit_case_sensitive() {
    let state = common::new_state();
    let result = common::process(&state, "Quit\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_quit_no_crlf() {
    let state = common::new_state();
    let result = common::process(&state, "quit");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_quit_extra_args() {
    let state = common::new_state();
    let result = common::process(&state, "quit now\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_quit_trailing_space() {
    let state = common::new_state();
    let result = common::process(&state, "quit \r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_quit_does_not_affect_state() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let mut output = Vec::new();
    process_input(&state, b"quit\r\n", &mut output);
    assert!(output.is_empty());

    let mut output = Vec::new();
    let still_alive = process_input(&state, b"get key\r\n", &mut output);
    assert!(still_alive);
    assert_eq!(output, b"VALUE key 0 5\r\nhello\r\nEND\r\n");
}
