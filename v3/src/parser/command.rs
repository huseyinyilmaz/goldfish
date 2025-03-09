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
    /*
    Add,
    Replace,
    Append,
    Prepend,
    Cas,
    */
    // Retrieval commands
    Get {
        key: Vec<u8>,
    },
    /*
    Gets,
    Gat,
    Gats,
    */
    // Meta comands
    Version,
    Quit,
    CannotParse(String),
}
