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
    Get {
        key: Vec<u8>,
    },
    Add,
    Replace,
    Append,
    Prepend,
    Cas,
    // Retrieval commands
    Gets,
    Gat,
    Gats,
    //
    Version,

    Quit,
    CannotParse(String),
}
