// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


// From George Marsaglia's "Xorshift RNGs", 2003
// https://www.jstatsoft.org/index.php/jss/article/view/v008i14/xorshift.pdf
pub fn cycle(mut y: u64) -> u64 {
    y ^= y << 13;
    y ^= y >> 7;
    y ^= y << 17;
    y
}

pub fn cycle_n(mut y: u64, n: u32) -> u64 {
    for _ in 0..n {
        y = cycle(y);
    }
    y
}

// From Allen B. Downey's "Generating Pseudo-random Floating-Point Values", 2007
// http://allendowney.com/research/rand/downey07randfloat.pdf
pub fn uniform_f64(exp_seed: u64, mantissa_seed: u64) -> f64 {
    let o = exponent_offset(exp_seed) as u64;
    let exp = 0x3FEu64 /*exponent for 1/2*/ - o;
    let mantissa_mask = (1u64 << 52) - 1;
    let bits = (mantissa_seed & mantissa_mask) | (exp << 52);
    f64::from_bits(bits)
}

fn exponent_offset(y: u64) -> u32 {
    let mask = (1u64 << 8) - 1;
    let mut coin_flips = 0u64;
    for i in 0..8 {
        let ones_in_byte = (y & (mask << (i << 3))).count_ones();
        coin_flips |= (ones_in_byte as u64 & 1) << i;
    }
    let top_bits = (y ^ (y >> 32)) | (1 << 32);
    coin_flips |= top_bits << 8;
    coin_flips.trailing_zeros()
}

// From George Marsaglia's "A Convenient Method for Generating Normal Variables", 1964
// https://epubs.siam.org/doi/abs/10.1137/1006063
// case_4 from http://www.wolframalpha.com/input/?i=N%5B2*(2*pi)%5E(-1%2F2)+*+(integrate+e%5E(-0.5+*+x%5E2)+from+3+to+infinity),+20%5D
pub fn normal_f64(seedling: u64) -> f64 {
    let case_1 = 0.8638;
    let case_2 =  0.1107;
    let case_4 = 0.00269979606326019;

    let seed = cycle(seedling.swap_bytes());
    let p = uniform_f64(seed, cycle(seed));
    let seed = cycle(cycle(seed));
    if p > (1f64 - case_1) {
        let u1 = uniform_f64(seed, cycle(seed));
        let seed2 = cycle(cycle(seed));
        let u2 = uniform_f64(seed2, cycle(seed2));
        let seed3 = cycle(cycle(seed2));
        let u3 = uniform_f64(seed3, cycle(seed3));
        2.0 * (u1 + u2 + u3 - 1.5)
    } else if p > (1f64 - case_1 - case_2) {
        let u1 = uniform_f64(seed, cycle(seed));
        let seed2 = cycle(cycle(seed));
        let u2 = uniform_f64(seed2, cycle(seed2));
        1.5 * (u1 + u2 - 1.0)
    } else if p > case_4 {
        let mut seed = seed;
        for _ in 0..100 {
            let u1 = uniform_f64(seed, cycle(seed));
            let seed2 = cycle(cycle(seed));
            let u2 = uniform_f64(seed2, cycle(seed2));
            let x = 6.0 * u1 - 3.0;
            let y = 0.358 * u2;
            if y < g3(x) {
                return x;
            } else {
                seed = cycle(cycle(seed2));
            }
        }
        panic!("normal_f64 case_3 drew unsuccessfully 100 times");
    } else {
        let mut seed = seed;
        for _ in 0..100 {
            let u1 = uniform_f64(seed, cycle(seed));
            let seed2 = cycle(cycle(seed));
            let u2 = uniform_f64(seed2, cycle(seed2));
            let seed3 = cycle(cycle(seed2));
            let mask = (1u64 << 32) - 1;
            let v1 = u1 * (-1f64).powi(((seed3 & mask).count_ones() & 1) as i32);
            let v2 = u2 * (-1f64).powi((((seed3 >> 32) & mask).count_ones() & 1) as i32);
            let r = v1.powi(2) + v2.powi(2);
            if r < 1.0 {
                let w = -(r.ln());
                let common = ((9.0 + 2.0 * w) / r).sqrt();
                let x = v1 * common;
                let y = v2 * common;
                if x.abs() > 3.0 {
                    return x;
                }
                if y.abs() > 3.0 {
                    return y;
                }
            }
            seed = cycle(seed3);
        }
        panic!("normal_f64 case_4 drew unsuccessfully 100 times");
    }
}

fn g3(x: f64) -> f64 {
    let abs_x = x.abs();
    let x2 = x.powi(2);
    let common = 17.49731196 * (-0.5 * x2).exp();
    let common1 = 2.15787544 * (1.5 - abs_x);
    let common2 = 2.36785163 * (3.0 - abs_x).powi(2);
    if abs_x < 1.0 {
        common - 4.73570326 * (3.0 - x2) - common1
    } else if abs_x < 1.5 {
        common - common1 - common2
    } else {
        common - common2
    }
}

