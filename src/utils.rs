use core::fmt::Write;

pub fn to_hex_string(bytes: &[u8]) -> String {
    let n = bytes.len();
    let mut hex_string = String::with_capacity(n * 2);
    for byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hex_string() {
        let bytes = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        assert_eq!(to_hex_string(&bytes), "000102030405060708090a0b0c0d0e0f");
        let bytes = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0,
            0xf0, 0xf0,
        ];
        assert_eq!(to_hex_string(&bytes), "102030405060708090a0b0c0d0e0f0f0");
        let bytes = [
            0x1f, 0x2f, 0x3f, 0x4f, 0x5f, 0x6f, 0x7f, 0x8f, 0x9f, 0xaf, 0xbf, 0xcf, 0xdf, 0xef,
            0xff,
        ];
        assert_eq!(to_hex_string(&bytes), "1f2f3f4f5f6f7f8f9fafbfcfdfefff");
    }
}
