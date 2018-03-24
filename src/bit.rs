pub const MASK_32: u64 = (1u64 << 32) - 1;

pub fn bottom_32(x: u64) -> u64 {
    MASK_32 & x
}

pub fn top_32(x: u64) -> u64 {
    MASK_32 & (x >> 32)
}
