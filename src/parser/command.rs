#[derive(Debug, PartialEq)]
pub enum Command {
    // Storage commands
    Set {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Add {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Replace {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Append {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Prepend {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Cas {
        key: Vec<u8>,
        flags: i32,
        timeout: i64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
        cas_unique: u64,
    },
    // Retrieval commands
    Get {
        keys: Vec<Vec<u8>>,
    },
    Gets {
        keys: Vec<Vec<u8>>,
    },
    Gat {
        timeout: i64,
        keys: Vec<Vec<u8>>,
    },
    Gats {
        timeout: i64,
        keys: Vec<Vec<u8>>,
    },
    // Arithmetic commands
    Incr {
        key: Vec<u8>,
        delta: u64,
        noreply: bool,
    },
    Decr {
        key: Vec<u8>,
        delta: u64,
        noreply: bool,
    },
    // Touch command
    Touch {
        key: Vec<u8>,
        timeout: i64,
        noreply: bool,
    },
    // Deletion commands
    Delete {
        key: Vec<u8>,
        noreply: bool,
    },
    FlushAll {
        delay: u64,
        noreply: bool,
    },
    // Statistics
    Stats {
        sub: Option<Vec<u8>>,
    },
    // Meta comands
    Version,
    Quit,
    Malformed,
    CannotParse(String),
}
