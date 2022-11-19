use core::fmt::Write;

pub fn to_hex_string(bytes: &[u8]) -> String {
    let n = bytes.len();
    let mut hex_string = String::with_capacity(n * 2);
    for byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}
