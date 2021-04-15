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
}

impl Command {
    pub fn remove_newline(s: &str) {}
}

// impl std::str::FromStr for Command {
//     fn from_str(s: &str) {

//     }
// }
