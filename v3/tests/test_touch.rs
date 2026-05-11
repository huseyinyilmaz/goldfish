mod common;

#[test]
fn test_touch_touched() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "touch key 100\r\n");
    assert_eq!(result, "TOUCHED\r\n");
}

#[test]
fn test_touch_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "touch key 100\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_touch_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "touch key 100 noreply\r\n");
    assert_eq!(result, "");
}

#[test]
fn test_touch_not_found_noreply() {
    let state = common::new_state();
    let result = common::process(&state, "touch key 100 noreply\r\n");
    assert_eq!(result, "");
}

#[test]
fn test_touch_key_too_long() {
    let state = common::new_state();
    let long_key = "k".repeat(251);
    let cmd = format!("touch {long_key} 100\r\n");
    let result = common::process(&state, &cmd);
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_touch_preserves_value() {
    let state = common::new_state();
    common::process(&state, "set key 42 0 5\r\nhello\r\n");
    common::process(&state, "touch key 100\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 42 5\r\nhello\r\nEND\r\n");
}
