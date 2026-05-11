mod common;

#[test]
fn test_flush_all_ok() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "flush_all\r\n");
    assert_eq!(result, "OK\r\n");
}

#[test]
fn test_flush_all_clears_keys() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "flush_all\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_flush_all_with_delay() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "flush_all 1\r\n");
    assert_eq!(result, "OK\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
    std::thread::sleep(std::time::Duration::from_secs(2));
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_flush_all_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "flush_all noreply\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_flush_all_with_delay_and_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "flush_all 1 noreply\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
    std::thread::sleep(std::time::Duration::from_secs(2));
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_flush_all_empty_state() {
    let state = common::new_state();
    let result = common::process(&state, "flush_all\r\n");
    assert_eq!(result, "OK\r\n");
}

#[test]
fn test_flush_all_multi() {
    let state = common::new_state();
    common::process(&state, "set k1 0 0 2\r\na\r\n");
    common::process(&state, "set k2 0 0 2\r\nb\r\n");
    common::process(&state, "set k3 0 0 2\r\nc\r\n");
    common::process(&state, "flush_all\r\n");
    let result = common::process(&state, "get k1 k2 k3\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_flush_all_then_set_again() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "flush_all\r\n");
    common::process(&state, "set key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}
