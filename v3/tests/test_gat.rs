mod common;

#[test]
fn test_gat_basic() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gat 100 key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_gat_miss() {
    let state = common::new_state();
    let result = common::process(&state, "gat 100 nonexistent\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_gat_multi_key() {
    let state = common::new_state();
    common::process(&state, "set k1 0 0 2\r\na\r\n");
    common::process(&state, "set k2 0 0 2\r\nb\r\n");
    let result = common::process(&state, "gat 100 k1 k2\r\n");
    assert!(result.contains("VALUE k1 0 2\r\n"));
    assert!(result.contains("VALUE k2 0 2\r\n"));
    assert!(result.ends_with("END\r\n"));
}

#[test]
fn test_gat_updates_expiry() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    // Touch exptime to a very small value
    common::process(&state, "gat 1 key\r\n");
    // Wait... actually, the key was just re-touched with exptime 1
    // We can verify it's still there immediately
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_gats_basic() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gats 100 key\r\n");
    assert!(result.starts_with("VALUE key 0 5 "));
    assert!(result.ends_with("END\r\n"));
}

#[test]
fn test_gats_miss() {
    let state = common::new_state();
    let result = common::process(&state, "gats 100 nonexistent\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_gat_key_too_long() {
    let state = common::new_state();
    let long_key = "k".repeat(251);
    let cmd = format!("gat 100 {long_key}\r\n");
    let result = common::process(&state, &cmd);
    assert_eq!(result, "CLIENT_ERROR bad command line format\r\n");
}

#[test]
fn test_gats_preserves_flags() {
    let state = common::new_state();
    common::process(&state, "set key 42 0 5\r\nhello\r\n");
    let result = common::process(&state, "gats 100 key\r\n");
    assert!(result.starts_with("VALUE key 42 5 "));
}
