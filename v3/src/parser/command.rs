#[derive(Debug, PartialEq)]
pub enum Command {
    // Storage commands
    Set {
        key: Vec<u8>,
        flags: i32,
        timeout: u64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    Add {
        key: Vec<u8>,
        flags: i32,
        timeout: u64,
        noreply: bool,
        value: Vec<u8>,
        value_size: u32,
    },
    /*
    Replace,
    Append,
    Prepend,
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
    CannotParse(String),
}
