mod common;

#[test]
fn test_stats_empty_state() {
    let state = common::new_state();
    let result = common::process(&state, "stats\r\n");
    assert!(result.starts_with("STAT "));
    assert!(result.ends_with("END\r\n"));
    assert!(result.contains("STAT curr_items 0\r\n"));
    assert!(result.contains("STAT total_items 0\r\n"));
    assert!(result.contains("STAT cmd_get 0\r\n"));
    assert!(result.contains("STAT cmd_set 0\r\n"));
    assert!(result.contains("STAT get_hits 0\r\n"));
    assert!(result.contains("STAT get_misses 0\r\n"));
    assert!(result.contains("STAT evictions 0\r\n"));
    assert!(result.contains("STAT bytes 0\r\n"));
}

#[test]
fn test_stats_after_set() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    let result = common::process(&state, "stats\r\n");
    assert!(result.contains("STAT curr_items 1\r\n"));
    assert!(result.contains("STAT total_items 1\r\n"));
    assert!(result.contains("STAT cmd_set 1\r\n"));
    assert!(result.contains("STAT cmd_get 0\r\n"));
}

#[test]
fn test_stats_after_get() {
    let state = common::new_state();
    common::process(&state, "get key\r\n");
    let result = common::process(&state, "stats\r\n");
    assert!(result.contains("STAT cmd_get 1\r\n"));
    assert!(result.contains("STAT get_misses 1\r\n"));
    assert!(result.contains("STAT get_hits 0\r\n"));
}

#[test]
fn test_stats_after_get_hit() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "get key\r\n");
    let result = common::process(&state, "stats\r\n");
    assert!(result.contains("STAT cmd_get 1\r\n"));
    assert!(result.contains("STAT get_hits 1\r\n"));
    assert!(result.contains("STAT get_misses 0\r\n"));
}

#[test]
fn test_stats_curr_items_after_delete() {
    let state = common::new_state();
    common::process(&state, "set key 0 0 5\r\nhello\r\n");
    common::process(&state, "delete key\r\n");
    let result = common::process(&state, "stats\r\n");
    assert!(result.contains("STAT curr_items 0\r\n"));
    assert!(result.contains("STAT total_items 1\r\n"));
}

#[test]
fn test_stats_with_subcommand() {
    let state = common::new_state();
    let result = common::process(&state, "stats items\r\n");
    assert_eq!(result, "END\r\n");
}

#[test]
fn test_stats_after_multiple_commands() {
    let state = common::new_state();
    common::process(&state, "set k1 0 0 2\r\na\r\n");
    common::process(&state, "set k2 0 0 2\r\nb\r\n");
    common::process(&state, "get k1\r\n");
    common::process(&state, "get k3\r\n");
    common::process(&state, "delete k1\r\n");
    let result = common::process(&state, "stats\r\n");
    assert!(result.contains("STAT curr_items 1\r\n"));
    assert!(result.contains("STAT total_items 2\r\n"));
    assert!(result.contains("STAT cmd_get 2\r\n"));
    assert!(result.contains("STAT cmd_set 2\r\n"));
    assert!(result.contains("STAT get_hits 1\r\n"));
    assert!(result.contains("STAT get_misses 1\r\n"));
}

#[test]
fn test_stats_has_required_fields() {
    let state = common::new_state();
    let result = common::process(&state, "stats\r\n");
    let required = [
        "STAT pid ",
        "STAT uptime ",
        "STAT time ",
        "STAT version ",
        "STAT pointer_size ",
        "STAT curr_items ",
        "STAT total_items ",
        "STAT bytes ",
        "STAT cmd_get ",
        "STAT cmd_set ",
        "STAT get_hits ",
        "STAT get_misses ",
        "STAT evictions ",
        "STAT limit_maxbytes ",
    ];
    for field in &required {
        assert!(result.contains(field), "Missing: {}", field);
    }
}
