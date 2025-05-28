use std::io::{self, Cursor, Read, Seek};

pub fn decompress_from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Vec<u8>> {
    rust_lzss::decompress(reader)
}

pub fn decompress_from_slice(slice: &[u8]) -> io::Result<Vec<u8>> {
    let mut cursor = Cursor::new(slice);
    decompress_from_reader(&mut cursor)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(
            decompress_from_slice(&[
                0x10, 0x14, 0x00, 0x00, 0x08, 0x61, 0x62, 0x63, 0x64, 0xD0, 0x03,
            ])
            .unwrap(),
            b"abcdabcdabcdabcdabcd"
        );
    }
}
