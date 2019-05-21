pub fn get_high_byte(value: u16) -> u16 {
    value & 0xFF00
}

pub fn get_low_byte(value: u16) -> u16 {
    value & 0x00FF
}
