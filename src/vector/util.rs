// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn reverse_digits(mut x: u32, digit_count: u32) -> u32 {
    let mut ret = 0u32;
    for i in 0..digit_count {
        ret = (ret << BITS) | (x & MASK);
        x = x >> BITS;
    }
    ret
}

pub fn significant_bits(x: u32) -> u32 {
    /*bits in a u32*/ 32 - x.leading_zeros()
}

pub fn digit_count(x: u32) -> u32 {
    (significant_bits(x) + BITS - 1) / BITS
}

pub fn trailing_zero_digit_count(x: u32) -> u32 {
    x.trailing_zeros() / BITS
}

pub fn root_content_count(tailoff: u32) -> u32 {
    let last_index = tailoff - 1;
    let dc = digit_count(last_index);
    let last_root_index = last_index >> (BITS * (dc - 1));
    last_root_index + 1
}

