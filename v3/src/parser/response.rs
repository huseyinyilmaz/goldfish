#[derive(Debug, PartialEq)]
pub enum CommandResponse {
    Version(String),
    Set(String),
    Get {
        key: Vec<u8>,
        data: Vec<u8>,
        flags: i32,
    },
    GetNotFound,
    Error,
}

impl CommandResponse {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            // b"VERSION <Version string>"
            Self::Version(version_string) => {
                [b"VERSION ", version_string.as_bytes(), b"\r\n"].concat()
            }
            Self::Set(response) => [response.as_bytes()].concat(),
            Self::Get { key, data, flags } => [
                b"VALUE ",
                key.as_slice(),
                b" ",
                flags.to_string().as_bytes(),
                b" ",
                data.len().to_string().as_bytes(),
                b"\r\n",
                data.as_slice(),
                b"\r\n",
                b"END\r\n",
            ]
            .concat(),
            Self::GetNotFound => b"END\r\n".to_vec(),
            // b"ERROR\r\n"
            Self::Error => b"ERROR\r\n".to_vec(),
        }
    }
}
