pub fn concat_bytes(left: u8, right: u8) -> u16 {
    ((left as u16) << 8) | (right as u16)
}
