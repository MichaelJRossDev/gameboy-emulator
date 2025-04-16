pub fn concat_bytes(left: u8, right: u8) -> u16 {
    ((left as u16) << 8) | (right as u16)
}

pub fn u16_from_little_endian(left: u8, right: u8) -> u16 {
    concat_bytes(right, left)
}

pub fn little_endian_from_u16(n: u16) -> (u8, u8) {
    let high_byte = ((n & 0xFF00) >> 8) as u8;
    let low_byte = (n & 0x00FF) as u8;

    (low_byte, high_byte)
}
