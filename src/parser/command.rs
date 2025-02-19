#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    // Storage commands
    Set {
        key: &'a [u8],
        flags: i32,
        timeout: i32,
        noreply: bool,
        value: &'a [u8],
        value_size: u32,
    },
    Add,
    Replace,
    Append,
    Prepend,
    Cas,
    // Retrieval commands
    Get,
    Gets,
    Gat,
    Gats,
    //
    Version,

    Quit,
    CannotParse(String),
}
