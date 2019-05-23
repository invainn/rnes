pub fn get_high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}

pub fn get_low_byte(value: u16) -> u8 {
    (value & 0x00FF) as u8
}
