mod common;

#[test]
fn test_gets_basic() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    assert!(result.starts_with("VALUE key 0 5 "));
    assert!(result.ends_with("END\r\n"));
}

#[test]
fn test_gets_has_cas_token() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "gets key\r\n");
    // VALUE line has 5 tokens: key, flags, bytes, cas_unique
    let line = result.split("\r\n").next().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "VALUE");
    assert_eq!(parts[1], "key");
    assert_eq!(parts[2], "0");
    assert_eq!(parts[3], "5");
    let cas_unique: u64 = parts[4].parse().unwrap();
    assert!(cas_unique > 0);
}

#[test]
fn test_gets_multi_key() {
    let state = common::new_state();
    common::process(&state, "set k1 0 0 2\r\na\r\n");
    common::process(&state, "set k2 0 0 2\r\nb\r\n");
    let result = common::process(&state, "gets k1 k2\r\n");
    assert!(result.contains("VALUE k1 0 2 "));
    assert!(result.contains("VALUE k2 0 2 "));
    assert!(result.ends_with("END\r\n"));
}

#[test]
fn test_gets_missing_key() {
    let state = common::new_state();
    let result = common::process(&state, "gets nonexistent\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_gets_cas_changes_after_update() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result1 = common::process(&state, "gets key\r\n");
    common::process(&state, "set key 0 0 5\r\nworld\r\n");
    let result2 = common::process(&state, "gets key\r\n");
    let cas1 = result1
        .split("\r\n")
        .next()
        .unwrap()
        .rsplit(' ')
        .next()
        .unwrap()
        .to_string();
    let cas2 = result2
        .split("\r\n")
        .next()
        .unwrap()
        .rsplit(' ')
        .next()
        .unwrap()
        .to_string();
    assert_ne!(cas1, cas2);
}

#[test]
fn test_gets_after_get() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let get_result = common::process(&state, "get key\r\n");
    let gets_result = common::process(&state, "gets key\r\n");
    assert!(get_result.starts_with("VALUE key 0 5\r\n"));
    assert!(gets_result.starts_with("VALUE key 0 5 "));
}
