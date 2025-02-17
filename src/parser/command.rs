#[derive(Debug, PartialEq)]
pub enum Command {
    // Storage commands
    Set,
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
