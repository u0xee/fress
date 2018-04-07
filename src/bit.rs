pub const MASK_32: u64 = (1u64 << 32) - 1;
pub const MASK_16: u64 = (1u64 << 16) - 1;
const MASK_SECOND_TOP_BYTE: u64 = (0xFFu64 << 48);

pub fn bottom_32(x: u64) -> u64 {
    MASK_32 & x
}

pub fn top_32(x: u64) -> u64 {
    MASK_32 & (x >> 32)
}

pub fn top_16(x: u64) -> u64 {
    MASK_16 & (x >> 48)
}

pub fn top_byte(x: u64) -> u8 {
    (x >> 56) as u8
}

pub fn second_top_byte(x: u64) -> u8 {
    (x << 8 >> 48) as u8
}

pub fn with_top_byte(x: u64, b: u8) -> u64 {
    ((b as u64) << 56) | (x << 8 >> 8)
}

pub fn with_second_top_byte(x: u64, b: u8) -> u64 {
    ((b as u64) << 48) | (x & !MASK_SECOND_TOP_BYTE)
}

pub fn clear_top(x: u64, bit_count: u8) -> u64 {
    x << bit_count >> bit_count
}

pub fn clear_bottom(x: u64, bit_count: u8) -> u64 {
    let bit_shift = bit_count % 64;
    x >> bit_shift << bit_shift
}

pub fn splice(top: u64, bottom: u64, top_bit_count: u8) -> u64 {
    clear_bottom(top, 64 - top_bit_count) |
        clear_top(bottom, top_bit_count)
}
