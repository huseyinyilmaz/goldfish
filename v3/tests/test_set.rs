mod common;

#[test]
fn test_set_basic() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_set_noreply() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5 noreply\r\nhello\r\n");
    assert_eq!(result, "");
}

#[test]
fn test_set_and_verify_via_get() {
    let state = common::new_state();
    common::process(&state, "set mykey 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "get mykey\r\n");
    assert_eq!(result, "VALUE mykey 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_overwrite() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "set key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_set_empty_value() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 0\r\n\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_set_empty_value_and_verify() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 0\r\n\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 0\r\n\r\nEND\r\n");
}

#[test]
fn test_set_with_flags() {
    let state = common::new_state();
    common::process(&state, "set key 42 0 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 42 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_with_large_flags() {
    let state = common::new_state();
    common::process(&state, "set key 65535 0 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 65535 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_with_timeout() {
    let state = common::new_state();
    common::process(&state, "set key 0 3600 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_zero_timeout_never_expires() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_binary_value() {
    let state = common::new_state();
    let binary: Vec<u8> = vec![0u8, 1, 2, 127, 128, 255, 13, 10];
    let mut input = b"set key 0 0 8\r\n".to_vec();
    input.extend_from_slice(&binary);
    input.extend_from_slice(b"\r\n");

    let mut output = Vec::new();
    goldfish::process_input(&state, &input, &mut output);
    assert_eq!(output, b"STORED\r\n");

    let mut output = Vec::new();
    goldfish::process_input(&state, b"get key\r\n", &mut output);
    assert!(output.starts_with(b"VALUE key 0 8\r\n"));
    assert!(output.windows(8).any(|w| w == binary.as_slice()));
}

#[test]
fn test_set_large_key() {
    let state = common::new_state();
    let key = "k".repeat(250);
    let cmd = format!("set {} 0 0 3\r\nval\r\n", key);
    let result = common::process(&state, &cmd);
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_set_multiple_spaces_between_args() {
    let state = common::new_state();
    let result = common::process(&state, "set   key   0   0   5\r\nhello\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_set_noreply_without_leading_space() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5noreply\r\nhello\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_malformed_short_value() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5\r\nab\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\nERROR\r\n");
}

#[test]
fn test_set_malformed_no_crlf_after_value() {
    let state = common::new_state();
    let mut buf = b"set key 0 0 5\r\nhello".to_vec();
    let mut output = Vec::new();
    let result = goldfish::process_input_buffered(&state, &mut buf, &mut output);
    assert!(result);
    assert!(output.is_empty());
    buf.extend_from_slice(b"\r\n");
    let mut output = Vec::new();
    goldfish::process_input_buffered(&state, &mut buf, &mut output);
    assert_eq!(output, b"STORED\r\n");
}

#[test]
fn test_set_malformed_missing_args() {
    let state = common::new_state();
    let result = common::process(&state, "set key\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_set_malformed_no_value_at_all() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_set_malformed_bad_flags() {
    let state = common::new_state();
    let result = common::process(&state, "set key abc 0 5\r\nhello\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\nERROR\r\n");
}

#[test]
fn test_set_malformed_bad_timeout() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 abc 5\r\nhello\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\nERROR\r\n");
}

#[test]
fn test_set_malformed_bad_bytes() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 abc\r\nhello\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\nERROR\r\n");
}

#[test]
fn test_set_pipeline() {
    let state = common::new_state();
    let result = common::process(&state, "set a 0 0 1\r\nx\r\nset b 0 0 1\r\ny\r\n");
    assert_eq!(result, "STORED\r\nSTORED\r\n");
}

#[test]
fn test_set_unicode_key() {
    let state = common::new_state();
    let key = "üñîçødê";
    let cmd = format!("set {} 0 0 3\r\nval\r\n", key);
    let result = common::process(&state, &cmd);
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_set_unicode_value() {
    let state = common::new_state();
    let val = "héllo";
    let cmd = format!("set key 0 0 6\r\n{}\r\n", val);
    let result = common::process(&state, &cmd);
    assert_eq!(result, "STORED\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 6\r\nhéllo\r\nEND\r\n");
}

#[test]
fn test_set_noreply_with_verify() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5 noreply\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_set_noreply_in_pipeline() {
    let state = common::new_state();
    let result = common::process(&state, "set a 0 0 1 noreply\r\nx\r\nset b 0 0 1\r\ny\r\n");
    assert_eq!(result, "STORED\r\n");
    let result = common::process(&state, "get a\r\n");
    assert_eq!(result, "VALUE a 0 1\r\nx\r\nEND\r\n");
}
