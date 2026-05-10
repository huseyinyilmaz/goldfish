pub fn handle_version(output: &mut Vec<u8>) {
    output.extend_from_slice(b"VERSION Goldfish 1.0\r\n");
}
