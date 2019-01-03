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

pub fn round_sum(mut x: u64) -> u64 {
    x = x.wrapping_add(x << 11);
    x = x.rotate_left(31);
    x = x.wrapping_add(x << 15);
    x = x.rotate_left(21);
    x = x.wrapping_add(x << 6);
    x
}

pub fn shear(x: u64) -> u64 {
    let even_mask = 0x55555555_55555555u64;
    let evens = x & even_mask;
    let odds = x & !even_mask;
    let y = odds | evens.rotate_left(13 * 2);
    y
}

pub fn h(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x = x.rotate_left(32);
    x ^= x << 5;
    x ^= x >> 3;
    x ^= NOISE[0];
    // 10 cycles

    //x = muck(x, 0xd542a8f6_287effc3u64, 0x5563911d_59dfa6aau64, 11, 7);
    x = round_sum(x);
    x = shear(x); // 4 cycles
    x = round_sum(x); // 8 cycles
    //x = muck(x, 0x5e5c9ec2_196a2463u64, 0x487cac60_5dec8032u64, 19, 13);
    //x = x.wrapping_add(x << 15);
    x
}

pub fn muck(x: u64, m: u64, c: u64, x_shift: u32, m_shift: u32) -> u64 {
    let mx = x.wrapping_mul(m) << m_shift;
    let y = x.wrapping_add(c) ^ (!x << x_shift);
    mx.wrapping_add(y)
}

pub fn avalanche(diff: u64) -> (u64, u64) {
    let mut seed = 0x12345678_9ABCDEF0u64;
    let mut flip = 0u64;
    let mut noflip = 0u64;
    for _ in 0..10 {
        let x = cycle_n(seed, 5).wrapping_add(cycle_n(seed, 8));
        seed = cycle_n(seed, 10);
        let y = x ^ diff;
        let xh = h(x);
        let yh = h(y);
        let hash_diff = xh ^ yh;
        flip |= hash_diff;
        noflip |= !hash_diff;
    }
    (flip, noflip)
}

pub fn bit_test(x: u64, bit_index: u32) -> u32 {
    ((x >> bit_index) & 1) as u32
}

pub fn one_bit_flip<H>(hash: H, xs: &[(u64, u64)], bit_index: u32) -> Vec<u32>
    where H: Fn(u64) -> u64 {
    let diff = 1u64 << bit_index;
    let mut bit_flips: Vec<u32> = vec![0; 64];
    for &(x, xh) in xs {
        let y = x ^ diff;
        let hash_diff = xh ^ hash(y);
        for i in 0..64u32 {
            bit_flips[i as usize] += bit_test(hash_diff, i);
        }
    }
    bit_flips
}

pub fn hash_each<H>(hash: H, states: &[u64]) -> Vec<(u64, u64)>
    where H: Fn(u64) -> u64 {
    let mut xs = vec!();
    for state in states {
        let x = *state;
        let h = hash(x);
        xs.push((x, h));
    }
    xs
}

pub fn random_states(count: u32, seed: u64) -> Vec<u64> {
    let mut state = cycle_n(seed, 10);
    let mut xs = vec!();
    for _ in 0..count {
        state = cycle_n(state, 3);
        xs.push(state);
    }
    xs
}

pub fn range_states(count: u32) -> Vec<u64> {
    (0..(count as u64)).collect()
}

pub fn counts_to_freqs(divisor: u32, counts: &[u32]) -> Vec<u32> {
    let d = divisor as f32;
    counts.iter().map(|c| {
        let f = (*c as f32) / d;
        (f * 100.0) as u32
    }).collect()
}

pub fn print_8x8(fs: &[u32]) {
    assert_eq!(64, fs.len());
    for i in 0..8 {
        let base = i << 3;
        let row = &fs[base..(base + 8)];
        println!("{:2} {:2} {:2} {:2} {:2} {:2} {:2} {:2}",
                 row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]);
    }
}


pub fn twist_mask(pattern: u64) -> u64 {
    let bottom_nineteen = pattern & (!0u64 >> (64 - 19));
    let bottom_cluster = bottom_nineteen << 2;
    let mask = bottom_cluster | bottom_cluster << 21 | bottom_cluster << 42;
    mask
}

pub fn twist(mask: u64, x: u64) -> u64 {
    let clusters = mask & x;
    let two_clusters_left = clusters << 21;
    let one_cluster_right = clusters >> 42;
    let template = !mask & x;
    let twisted = template | two_clusters_left | one_cluster_right;
    twisted
}

pub fn twist_and_turn(x: u64, pattern: u64, turns: u32, rotate_bits: u32) -> u64 {
    let mask = twist_mask(pattern);
    let mut t = twist(mask, x);
    for _ in 0..turns {
        t = t.rotate_right(rotate_bits);
        t = twist(mask, t);
    }
    t
}

pub fn index_mapping(index: u32, two_spin: u32, four_spin: u32, eight_spin: u32) -> u32 {
    let mut i = index;
    if i % 2 == 0 {
        i = (i + (two_spin * 2)) % 64;
    }
    if i % 4 <= 1 {
        i = (i + (four_spin * 4)) % 64;
    }
    match i % 8 {
        1 | 3 | 4 | 6 | 7 => {
            i = (i + (eight_spin * 8)) % 64;
        },
        _ => {},
    }
    i
}

pub fn nearest_pair_dist(s: &[u32]) -> u32 {
    let mut d = s.first().unwrap() + 64 - s.last().unwrap();
    for i in 1..s.len() {
        let f = s[i] - s[i - 1];
        if f < d {
            d = f;
        }
    }
    d
}

pub fn neighbor_mapping_distance(two_spin: u32, four_spin: u32, eight_spin: u32) -> u32 {
    let mut v: Vec<u32> = vec!();
    for i in 0..64 {
        v.push(index_mapping(i, two_spin, four_spin, eight_spin));
    }
    let mut ds: Vec<u32> = vec!();
    let mut buf: [u32; 8] = [0; 8];
    for i in 0..(64 - 7) {
        for j in 0..8usize {
            buf[j] = v[i + j];
        }
        buf.sort();
        let d = nearest_pair_dist(&buf);
        ds.push(d);
    }
    ds.sort();
    let ret = ds[5];
    ret
}

pub fn scramble(x: u64) -> u64 {
    let even_mask = 0x55555555_55555555u64;
    let evens = x & even_mask;
    let odds = x & !even_mask;
    let y = odds | evens.rotate_left(14 * 2);

    let nibble_mask = 0x33333333_33333333u64;
    let low_pairs = y & nibble_mask;
    let high_pairs = y & !nibble_mask;
    let z = high_pairs | low_pairs.rotate_left(11 * 4);

    let byte_mask = 0xDADADADA_DADADADAu64;
    let moving_bits = z & byte_mask;
    let stationary = z & !byte_mask;
    let ret = stationary | moving_bits.rotate_left(7 * 8);
    ret
}

pub fn turbine(x: u64) -> u64 {
    let discs: [u64; 4] = [0x11111111_11111111, 0x22222222_22222222, 0x44444444_44444444, 0x88888888_88888888];
    let votes = {
        let y = x ^ x >> 32;
        y ^ y >> 16
    };
    let mut res = 0u64;
    for i in 0..4u32 {
        let rotate_places = 0xF & votes >> (i << 2);
        let rotate_bits = (rotate_places as u32) << 2;
        let d = (x & discs[i as usize]).rotate_left(rotate_bits);
        res |= d;
    }
    res
}

pub const NOISE: [u64; 4] = [0x082efa98_ec4e6c89, 0x452821e6_38d01377, 0xbe5466cf_34e90c6c, 0xc0ac29b7_c97c50dd];
pub const PATTERN_MATERIAL: u64 = 0x3f84d5b5_b5470917;

pub fn hash_64(x: u64, byte_count: u32) -> u32 {
    let y = x << 8 | (byte_count as u64);
    let (a, b) = hash_raw_256(x, x, x, y);
    a as u32
}

pub fn hash_128(x: u64, y: u64, byte_count: u32) -> u32 {
    let z = y << 8 | (byte_count as u64);
    let (a, b) = hash_raw_256(x, y, x, z);
    a as u32
}

pub fn hash_192(x: u64, y: u64, z: u64, byte_count: u32) -> u32 {
    let w = x << 8 | (byte_count as u64);
    let (a, b) = hash_raw_256(x, y, z, w);
    a as u32
}

pub fn hash_256(x: u64, y: u64, z: u64, w: u64, byte_count: u32) -> u32 {
    let v = w << 8 | (byte_count as u64);
    let (a, b) = hash_raw_256(x, y, z, v);
    a as u32
}

pub const PI: [u64; 4] = [0x243f6a88_85a308d3, 0x13198a2e_03707344, 0xa4093822_299f31d0, 0x082efa98_ec4e6c89];

pub fn hash_raw_256(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64) {
    a = a.wrapping_add(PI[0]); b = b.wrapping_add(PI[1]);
    c = c.wrapping_add(PI[2]); d = d.wrapping_add(PI[3]);
    let (e, f, g, h) = mix(a, b, c, d);
    end(e, f, g, h)
}

pub fn mix(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64, u64, u64) {
    c = c.rotate_left(50);  c = c.wrapping_add(d);  a ^= c;
    d = d.rotate_left(52);  d = d.wrapping_add(a);  b ^= d;
    a = a.rotate_left(30);  a = a.wrapping_add(b);  c ^= a;
    b = b.rotate_left(41);  b = b.wrapping_add(c);  d ^= b;
    c = c.rotate_left(54);  c = c.wrapping_add(d);  a ^= c;
    d = d.rotate_left(48);  d = d.wrapping_add(a);  b ^= d;
    a = a.rotate_left(38);  a = a.wrapping_add(b);  c ^= a;
    b = b.rotate_left(37);  b = b.wrapping_add(c);  d ^= b;
    c = c.rotate_left(62);  c = c.wrapping_add(d);  a ^= c;
    d = d.rotate_left(34);  d = d.wrapping_add(a);  b ^= d;
    a = a.rotate_left(5);   a = a.wrapping_add(b);  c ^= a;
    b = b.rotate_left(36);  b = b.wrapping_add(c);  d ^= b;
    (a, b, c, d)
}

pub fn end(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64) {
    /*
    d ^= c;  c = c.rotate_left(15);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(52);  a = a.wrapping_add(d);
    b ^= a;  a = a.rotate_left(26);  b = b.wrapping_add(a);
    c ^= b;  b = b.rotate_left(51);  c = c.wrapping_add(b);
    d ^= c;  c = c.rotate_left(28);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(9);   a = a.wrapping_add(d);
    */
    b ^= a;  a = a.rotate_left(47);  b = b.wrapping_add(a);
    c ^= b;  b = b.rotate_left(54);  c = c.wrapping_add(b);
    d ^= c;  c = c.rotate_left(32);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(25);  a = a.wrapping_add(d);
    b ^= a;  a = a.rotate_left(63);  b = b.wrapping_add(a);
    (a, b)
}


