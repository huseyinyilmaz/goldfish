mod common;

#[test]
fn test_add_stored() {
    let state = common::new_state();
    let result = common::process(&state, "add key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_add_not_stored() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "add key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "NOT_STORED\r\n");
}

#[test]
fn test_add_noreply() {
    let state = common::new_state();
    let result = common::process(&state, "add key 0 0 5 noreply\r\nhello\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_add_existing_key_value_unchanged() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "add key 0 0 6\r\nworld!\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nhello\r\nEND\r\n");
}

#[test]
fn test_replace_not_stored() {
    let state = common::new_state();
    let result = common::process(&state, "replace key 0 0 5\r\nhello\r\n");
    assert_eq!(result, "NOT_STORED\r\n");
}

#[test]
fn test_replace_stored() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "replace key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_replace_updates_value() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "replace key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_replace_noreply() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "replace key 0 0 5 noreply\r\nworld\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 5\r\nworld\r\nEND\r\n");
}

#[test]
fn test_append_not_stored() {
    let state = common::new_state();
    let result = common::process(&state, "append key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "NOT_STORED\r\n");
}

#[test]
fn test_append_stored() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "append key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_append_updates_value() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "append key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 10\r\nhelloworld\r\nEND\r\n");
}

#[test]
fn test_prepend_not_stored() {
    let state = common::new_state();
    let result = common::process(&state, "prepend key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "NOT_STORED\r\n");
}

#[test]
fn test_prepend_stored() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "prepend key 0 0 5\r\nworld\r\n");
    assert_eq!(result, "STORED\r\n");
}

#[test]
fn test_prepend_updates_value() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "prepend key 0 0 5\r\nworld\r\n");
    let result = common::process(&state, "get key\r\n");
    assert_eq!(result, "VALUE key 0 10\r\nworldhello\r\nEND\r\n");
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
fn test_incr_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "incr counter 1\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_decr_not_found() {
    let state = common::new_state();
    let result = common::process(&state, "decr counter 1\r\n");
    assert_eq!(result, "NOT_FOUND\r\n");
}

#[test]
fn test_incr_basic() {
    let state = common::new_state();
    common::process(&state, "set counter 0 0 1\r\n5\r\n");
    let result = common::process(&state, "incr counter 3\r\n");
    assert_eq!(result, "8\r\n");
}

#[test]
fn test_decr_basic() {
    let state = common::new_state();
    common::process(&state, "set counter 0 0 1\r\n5\r\n");
    let result = common::process(&state, "decr counter 2\r\n");
    assert_eq!(result, "3\r\n");
}

#[test]
fn test_decr_clamp_to_zero() {
    let state = common::new_state();
    common::process(&state, "set counter 0 0 1\r\n5\r\n");
    let result = common::process(&state, "decr counter 10\r\n");
    assert_eq!(result, "0\r\n");
}

#[test]
fn test_incr_noreply() {
    let state = common::new_state();
    common::process(&state, "set counter 0 0 1\r\n5\r\n");
    let result = common::process(&state, "incr counter 3 noreply\r\n");
    assert_eq!(result, "");
    let result = common::process(&state, "get counter\r\n");
    assert_eq!(result, "VALUE counter 0 1\r\n8\r\nEND\r\n");
}

#[test]
fn test_decr_from_set_string_fails() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "incr key 1\r\n");
    assert_eq!(
        result,
        "CLIENT_ERROR cannot increment or decrement non-numeric value\r\n"
    );
}

#[test]
fn test_touch_not_implemented() {
    let state = common::new_state();
    let result = common::process(&state, "touch key 3600\r\n");
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
