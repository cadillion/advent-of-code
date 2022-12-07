pub fn parse_str_fast(s: &str) -> u32 {
    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as u32)
}

pub fn parse_bytes_fast(s: &[u8]) -> u32 {
    s.into_iter().fold(0, |a, c| a * 10 + (c & 0x0f) as u32)
}
