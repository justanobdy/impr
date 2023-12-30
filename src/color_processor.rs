const BIT16_TO_BIT5_CONVERSION_FACTOR: f32 = 31.0 / 255.0;

pub fn bit16_to_bit5(num: u16) -> u16 {
    ((num as f32) * BIT16_TO_BIT5_CONVERSION_FACTOR) as u16
}

//rrrrrgggggbbbbba
pub fn create_16bit_color(r: u16, g: u16, b: u16, _a: u16) -> u16 {
    (bit16_to_bit5(r)) | ((bit16_to_bit5(g)) << 5) | ((bit16_to_bit5(b)) << 10)
}

//arrrrrgggggbbbbb
pub fn create_16bit_color_argb16(r: u16, g: u16, b: u16, a: u16) -> u16 {
    bit16_to_bit5(r)
        | (bit16_to_bit5(g) << 5)
        | (bit16_to_bit5(b) << 10)
        | ((if a > 0 { 1 } else { 0 }) << 15)
}

//rrrrrrrrggggggggbbbbbbbbaaaaaaaa
pub fn create_32bit_color(r: u32, g: u32, b: u32, a: u32) -> u32 {
    ((r & 0xFF) << 24) | ((g & 0xFF) << 16) | ((b & 0xFF) << 8) | (a & 0xFF)
}
