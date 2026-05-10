mod common;

#[test]
fn test_version_basic() {
    let state = common::new_state();
    let result = common::process(&state, "version\r\n");
    assert_eq!(result, "VERSION Goldfish 1.0\r\n");
}

#[test]
fn test_version_pipeline() {
    let state = common::new_state();
    let result = common::process(&state, "version\r\nversion\r\n");
    assert_eq!(result, "VERSION Goldfish 1.0\r\nVERSION Goldfish 1.0\r\n");
}

#[test]
fn test_version_after_set() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "version\r\n");
    assert_eq!(result, "VERSION Goldfish 1.0\r\n");
}

#[test]
fn test_version_in_pipeline_with_set_and_get() {
    let state = common::new_state();
    let result = common::process(
        &state,
        "version\r\nset key 0 0 5\r\nhello\r\nget key\r\nversion\r\n",
    );
    assert_eq!(
        result,
        "VERSION Goldfish 1.0\r\nSTORED\r\nVALUE key 0 5\r\nhello\r\nEND\r\nVERSION Goldfish 1.0\r\n"
    );
}

#[test]
fn test_version_case_sensitive() {
    let state = common::new_state();
    let result = common::process(&state, "Version\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_version_no_crlf() {
    let state = common::new_state();
    let result = common::process(&state, "version");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_version_trailing_garbage() {
    let state = common::new_state();
    let result = common::process(&state, "version x\r\n");
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}
