#[derive(Debug, PartialEq)]
pub enum CommandResponse {
    Version(String),
    UnhandledCommand(String),
}

impl CommandResponse {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            Self::Version(version_string) => [version_string.as_bytes(), b"\r\n"].concat(),
            Self::UnhandledCommand(response_string) => [response_string.as_bytes(), b"\r\n"].concat(),
        }
    }
}

