use goldfish::process_input_buffered;

mod common;

#[test]
fn test_empty_input() {
    let state = common::new_state();
    let (ok, output) = common::process_raw(&state, b"");
    assert!(ok);
    assert!(output.is_empty());
}

#[test]
fn test_unknown_command() {
    let state = common::new_state();
    let result = common::process(&state, "foobar\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_unknown_command_no_crlf() {
    let state = common::new_state();
    let mut buf = b"foobar".to_vec();
    let mut output = Vec::new();
    process_input_buffered(&state, &mut buf, &mut output);
    assert_eq!(output, b"ERROR\r\n");
}

#[test]
fn test_empty_line_is_error() {
    let state = common::new_state();
    let result = common::process(&state, "\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_case_sensitive_set() {
    let state = common::new_state();
    let result = common::process(&state, "SET key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\nERROR\r\n");
}

#[test]
fn test_case_sensitive_get() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "GET key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_pipeline_set_get() {
    let state = common::new_state();
    let result = common::process(&state, "set key 0 0 5\r\nhello\r\nget key\r\n");
    assert_eq!(result, "STORED\r\nVALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_pipeline_many_commands() {
    let state = common::new_state();
    let input = "version\r\nset a 0 0 1\r\nx\r\nget a\r\nversion\r\nset b 0 0 1\r\ny\r\nget b\r\n";
    let result = common::process(&state, input);
    assert_eq!(
        result,
        "VERSION Goldfish 1.0\r\nSTORED\r\nVALUE a 0 1\r\nx\r\nEND\r\n\
         VERSION Goldfish 1.0\r\nSTORED\r\nVALUE b 0 1\r\ny\r\nEND\r\n"
    );
}
