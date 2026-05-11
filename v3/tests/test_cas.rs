mod common;

#[test]
fn test_cas_stored() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    assert!(result.starts_with("VALUE key 0 5 "));
    let cas_str = result
        .strip_prefix("VALUE key 0 5 ")
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap();
    let cas_input = format!("cas key 0 0 5 {cas_str}\r\nworld\r\n");
    let result = common::process(&state, &cas_input);
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_cas_value_updated() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    let cas_str = result
        .strip_prefix("VALUE key 0 5 ")
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap();
    let cas_input = format!("cas key 0 0 5 {cas_str}\r\nworld\r\n");
    common::process(&state, &cas_input);
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_cas_exists() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "cas key 0 0 5 999\r\nworld\r\n");
    assert_eq!(result, "EXISTS\r\n");
}

#[test]
fn test_cas_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "cas key 0 0 5 1\r\nworld\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_cas_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    let cas_str = result
        .strip_prefix("VALUE key 0 5 ")
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap();
    let cas_input = format!("cas key 0 0 5 {cas_str} noreply\r\nworld\r\n");
    let result = common::process(&state, &cas_input);
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_cas_exists_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "cas key 0 0 5 999 noreply\r\nworld\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_cas_not_found_noreply() {
    let state = common::new_state();
    let result = common::process(&state, "cas key 0 0 5 1 noreply\r\nworld\r\n");
    assert_eq!(result, "");
}

#[test]
fn test_cas_key_too_long() {
    let state = common::new_state();
    let long_key = "k".repeat(251);
    let cmd = format!("cas {long_key} 0 0 5 1\r\nhello\r\n");
    let result = common::process(&state, &cmd);
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_cas_updates_cas() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    let cas1 = result
        .strip_prefix("VALUE key 0 5 ")
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap()
        .to_string();
    let cas_input = format!("cas key 0 0 5 {cas1}\r\nworld\r\n");
    common::process(&state, &cas_input);
    let result = common::process(&state, "gets key\r\n");
    let cas2 = result
        .strip_prefix("VALUE key 0 5 ")
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap();
    assert_ne!(cas1, cas2);
}
