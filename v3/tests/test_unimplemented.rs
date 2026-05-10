mod common;

#[test]
fn test_add_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "add key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_replace_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "replace key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_append_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "append key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_prepend_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "prepend key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_cas_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "cas key 0 0 5 0\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_gets_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "gets key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_gat_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "gat 3600 key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_gats_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "gats 3600 key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_delete_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "delete key\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_delete_with_noreply_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "delete key noreply\r\n");
    assert_eq!(result, "");
}

#[test]
fn test_delete_found() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "delete key\r\n");
    assert_eq!(result, "DELETED\r\n");
}

#[test]
fn test_delete_twice() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "delete key\r\n");
    assert_eq!(result, "DELETED\r\n");
    let result = common::process(&state, "delete key\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_incr_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "incr counter 1\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_decr_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "decr counter 1\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_touch_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "touch key 3600\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_stats_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "stats\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_flush_all_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "flush_all\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_mg_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "mg key v\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_ms_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "ms key 5 T3600\r\nhello\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_md_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "md key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_ma_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "ma key D5 v\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_mn_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "mn\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_me_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "me key\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_slabs_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "slabs reassign -1 5\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_lru_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "lru mode flat\r\n");
    assert_eq!(result, "ERROR\r\n");
}

#[test]
fn test_watch_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "watch fetchers\r\n");
    assert_eq!(result, "ERROR\r\n");
}
