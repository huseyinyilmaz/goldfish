#[derive(Debug, PartialEq)]
pub enum CommandResponse {
    Version(String),
    Set(String),
    Error,
}

impl CommandResponse {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            // b"VERSION <Version string>"
            Self::Version(version_string) => [b"VERSION ", version_string.as_bytes(), b"\r\n"].concat(),
            Self::Set(response) => [response.as_bytes()].concat(),
            // b"ERROR\r\n"
            Self::Error => b"ERROR\r\n".to_vec(),
        }
    }
}

