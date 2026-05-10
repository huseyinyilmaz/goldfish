use std::str;

pub fn raw_string_to_string(raw_input: &[u8]) -> &str {
    match str::from_utf8(raw_input) {
        // TODO remove empty \0 characters from string
        Ok(s) => s,
        _ => "Unparsable Input String",
    }
}

pub fn has_control_chars(bytes: &[u8]) -> bool {
    bytes.iter().any(|&b| b < 0x20)
}
