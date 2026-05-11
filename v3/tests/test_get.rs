mod common;

#[test]
fn test_get_found() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_get_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "get nonexistent\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_get_after_noreply_set() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5 noreply\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_get_returns_correct_flags() {
    let state = common::new_state();
    common::process(&state, "set key 99 0 5\r\nhello\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 99 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_get_after_overwrite() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "set key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_get_multiple_different_keys() {
    let state = common::new_state();
    common::process(&state, "set a 0 0 1\r\nx\r\n");
    common::process(&state, "set b 0 0 1\r\ny\r\n");
    let result_a = common::process(&state, "get a\r\n");
    assert_eq!(result_a, "VALUE a 0 1\r\nx\r\nEND\r\n");
    let result_b = common::process(&state, "get b\r\n");
    assert_eq!(result_b, "VALUE b 0 1\r\ny\r\nEND\r\n");
}

#[test]
fn test_get_empty_value() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 0\r\n\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 0\r\n\r\nEND\r\n");
}

#[test]
fn test_get_empty_state() {
    let state = common::new_state();
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_get_binary_value() {
    let state = common::new_state();
    let bin: &[u8] = &[0x00, 0x01, 0xff];
    let mut input = b"set key 0 0 3\r\n".to_vec();
    input.extend_from_slice(bin);
    input.extend_from_slice(b"\r\n");
    let mut output = Vec::new();
    goldfish::process_input(&state, &input, &mut output);

    let mut output = Vec::new();
    goldfish::process_input(&state, b"get key\r\n", &mut output);
    assert!(output.starts_with(b"VALUE key 0 3\r\n"));
    assert!(output.windows(3).any(|w| w == bin));
    assert!(output.ends_with(b"\r\nEND\r\n"));
}

#[test]
fn test_get_unicode_value() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 6\r\nhéllo\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 6\r\nhéllo\r\nEND\r\n");
}

#[test]
fn test_get_after_delete() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "delete key\r\n");
    assert_eq!(result, "DELETED\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_get_malformed_no_key() {
    let state = common::new_state();
    let result = common::process(&state, "get\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_get_multiple_keys_found() {
    let state = common::new_state();
    common::process(&state, "set a 0 0 1\r\nx\r\n");
    common::process(&state, "set b 1 0 1\r\ny\r\n");
    let result = common::process(&state, "get a b\r\n");
    assert_eq!(result, "VALUE a 0 1\r\nx\r\nVALUE b 1 1\r\ny\r\nEND\r\n");
}

#[test]
fn test_get_multiple_keys_some_missing() {
    let state = common::new_state();
    common::process(&state, "set a 0 0 1\r\nx\r\n");
    let result = common::process(&state, "get a b\r\n");
    assert_eq!(result, "VALUE a 0 1\r\nx\r\nEND\r\n");
}

#[test]
fn test_get_multiple_keys_all_missing() {
    let state = common::new_state();
    let result = common::process(&state, "get a b\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_get_multiple_keys_three_keys() {
    let state = common::new_state();
    common::process(&state, "set x 0 0 1\r\na\r\n");
    common::process(&state, "set y 0 0 1\r\nb\r\n");
    common::process(&state, "set z 0 0 1\r\nc\r\n");
    let result = common::process(&state, "get x y z\r\n");
    assert_eq!(
        result,
        "VALUE x 0 1\r\na\r\nVALUE y 0 1\r\nb\r\nVALUE z 0 1\r\nc\r\nEND\r\n"
    );
}


