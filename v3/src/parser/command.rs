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
    /*
    Cas,
    */
    // Retrieval commands
    Get {
        keys: Vec<Vec<u8>>,
    },
    /*
    Gets,
    Gat,
    Gats,
    */
    // Deletion commands
    Delete {
        key: Vec<u8>,
        noreply: bool,
    },
    // Meta comands
    Version,
    Quit,
    Malformed,
    CannotParse(String),
}
