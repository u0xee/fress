// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Spooky Hash

// 128 bit seed to 128 bit hash
// 64 bit seed to 64 bit hash
// 32 bit seed to 32 bit hash
// State machine. Init with seed, feed message chunks, finish
//   unhashed data buffer, internal state, byte_count so far
// Mix 12, End 12
// Mix short, end



// ShortMix from Bob Jenkins' SpookyHash.
// http://burtleburtle.net/bob/hash/spooky.html
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

// ShortEnd from Bob Jenkins' SpookyHash.
pub fn end(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64) {
    d ^= c;  c = c.rotate_left(15);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(52);  a = a.wrapping_add(d);
    b ^= a;  a = a.rotate_left(26);  b = b.wrapping_add(a);
    c ^= b;  b = b.rotate_left(51);  c = c.wrapping_add(b);
    d ^= c;  c = c.rotate_left(28);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(9);   a = a.wrapping_add(d);

    b ^= a;  a = a.rotate_left(47);  b = b.wrapping_add(a);
    c ^= b;  b = b.rotate_left(54);  c = c.wrapping_add(b);
    d ^= c;  c = c.rotate_left(32);  d = d.wrapping_add(c);
    a ^= d;  d = d.rotate_left(25);  a = a.wrapping_add(d);
    b ^= a;  a = a.rotate_left(63);  b = b.wrapping_add(a);
    (a, b)
}


fn round(a: &[u64], e: &mut [u64], c: &mut [u64]) {
    let d = [c[4] ^ c[1].rotate_left(1),
             c[0] ^ c[2].rotate_left(1),
             c[1] ^ c[3].rotate_left(1),
             c[2] ^ c[4].rotate_left(1),
             c[3] ^ c[0].rotate_left(1)];
    c[0] = 0; c[1] = 0; c[2] = 0; c[3] = 0; c[4] = 0;

    macro_rules! row { ($i:literal, $d:literal) => {
        let b0 = (a[PI[$i + 0]] ^ d[($d + 0) % 5]).rotate_left(RHO[PI[$i + 0]]);
        let b1 = (a[PI[$i + 1]] ^ d[($d + 1) % 5]).rotate_left(RHO[PI[$i + 1]]);
        let b2 = (a[PI[$i + 2]] ^ d[($d + 2) % 5]).rotate_left(RHO[PI[$i + 2]]);
        let b3 = (a[PI[$i + 3]] ^ d[($d + 3) % 5]).rotate_left(RHO[PI[$i + 3]]);
        let b4 = (a[PI[$i + 4]] ^ d[($d + 4) % 5]).rotate_left(RHO[PI[$i + 4]]);

        let e0 = b0 ^ (!b1 & b2); e[$i + 0] = e0; c[0] ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[$i + 1] = e1; c[1] ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[$i + 2] = e2; c[2] ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[$i + 3] = e3; c[3] ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[$i + 4] = e4; c[4] ^= e4;
    } }
    row!( 0, 0);
    row!( 5, 3);
    row!(10, 1);
    row!(15, 4);
    row!(20, 2);
}

const RHO: [u32; 25] = [
     0,  1, 62, 28, 27,
    36, 44,  6, 55, 20,
     3, 10, 43, 25, 39,
    41, 45, 15, 21,  8,
    18,  2, 61, 56, 14,
];
const PI: [usize; 25] = [
    0, 1 + 5, 2 + 10, 3 + 15, 4 + 20,
    3, 4 + 5, 0 + 10, 1 + 15, 2 + 20,
    1, 2 + 5, 3 + 10, 4 + 15, 0 + 20,
    4, 0 + 5, 1 + 10, 2 + 15, 3 + 20,
    2, 3 + 5, 4 + 10, 0 + 15, 1 + 20,
];
const RC: [u64; 24] = [
    0x00000000_00000001, 0x00000000_00008082, 0x80000000_0000808A, 0x80000000_80008000,
    0x00000000_0000808B, 0x00000000_80000001, 0x80000000_80008081, 0x80000000_00008009,
    0x00000000_0000008A, 0x00000000_00000088, 0x00000000_80008009, 0x00000000_8000000A,
    0x00000000_8000808B, 0x80000000_0000008B, 0x80000000_00008089, 0x80000000_00008003,
    0x80000000_00008002, 0x80000000_00000080, 0x00000000_0000800A, 0x80000000_8000000A,
    0x80000000_80008081, 0x80000000_00008080, 0x00000000_80000001, 0x80000000_80008008,
];

pub fn permute(a: &mut [u64], rounds: u8) {
    assert!(rounds <= 24 && rounds & 1 == 0);
    assert_eq!(a.len(), 25);
    let mut e = [0u64; 25];
    let mut c = [0u64; 5];
    if cfg!(target_endian = "big") { swap_bytes(a) }
    sum_columns(a, &mut c);

    let first_idx = 24 - rounds;
    for pair in 0..(rounds >> 1) {
        let idx = (first_idx + (pair << 1)) as usize;
        {
            round(a, &mut e, &mut c);
            let rc = RC[idx];
            e[0] ^= rc; c[0] ^= rc;
        }
        {
            round(&e, a, &mut c);
            let rc = RC[idx + 1];
            a[0] ^= rc; c[0] ^= rc;
        }
    }
    if cfg!(target_endian = "big") { swap_bytes(a) }
}

fn sum_columns(a: &[u64], c: &mut [u64]) {
    c[0] = a[0] ^ a[5] ^ a[10] ^ a[15] ^ a[20];
    c[1] = a[1] ^ a[6] ^ a[11] ^ a[16] ^ a[21];
    c[2] = a[2] ^ a[7] ^ a[12] ^ a[17] ^ a[22];
    c[3] = a[3] ^ a[8] ^ a[13] ^ a[18] ^ a[23];
    c[4] = a[4] ^ a[9] ^ a[14] ^ a[19] ^ a[24];
}
fn swap_bytes(a: &mut [u64]) {
    for i in 0..a.len() {
        a[i] = a[i].swap_bytes();
    }
}

/// Supporting SHA 256, 512, and k12
#[derive(Debug)]
pub struct Sponge {
    pub rate: u8,
    pub rounds: u8,
    pub fill: u8,
    pub state: [u64; 25],
}

mod suffix {
    pub const SHA:      u8 = 0x06;
    pub const K12_ONE:  u8 = 0x07;
    pub const K12_MANY: u8 = 0x06;
    pub const K12_LEAF: u8 = 0x0B;
}
fn xor(a: &mut [u8], b: &[u8]) {
    for i in 0..a.len() {
        a[i] ^= b[i];
    }
}
impl Sponge {
    pub fn new_k12() -> Sponge { Sponge::new(168, 12) }
    pub fn new_256() -> Sponge { Sponge::new(136, 24) }
    pub fn new_512() -> Sponge { Sponge::new( 72, 24) }
    pub fn new(rate: u8, rounds: u8) -> Sponge {
        assert!(72 <= rate && rate <= 168);
        assert!(rounds <= 24 && rounds & 1 == 0);
        Sponge { rate, rounds, fill: 0, state: [0u64; 25] }
    }
    pub fn reset(&mut self) {
        self.fill = 0;
        self.state = [0u64; 25];
    }
    pub fn state_as_buf(&mut self) -> &mut [u8] {
        use std::slice::from_raw_parts_mut;
        unsafe {
            let byte_ptr = self.state.as_mut_ptr() as *mut u8;
            let b = from_raw_parts_mut(byte_ptr, self.rate as usize);
            &mut b[(self.fill as usize)..]
        }
    }
    pub fn ingest(&mut self, bytes: &[u8]) {
        let mut b = bytes;
        loop {
            let buf = self.state_as_buf();
            if b.len() < buf.len() {
                xor(&mut buf[..b.len()], b);
                self.fill += b.len() as u8;
                break;
            }
            xor(buf, &b[..buf.len()]);
            b = &b[buf.len()..];
            permute(&mut self.state, self.rounds);
            self.fill = 0;
        }
    }
    pub fn finish(&mut self, suffix: u8) {
        assert_ne!(suffix, 0);
        assert_eq!(suffix >> 7, 0);
        let buf = self.state_as_buf();
        buf[0] ^= suffix;
        buf[buf.len() - 1] ^= 0x80;
        permute(&mut self.state, self.rounds);
        self.fill = 0;
    }
    pub fn extract_32(&mut self) -> [u8; 32] {
        assert_eq!(self.fill, 0);
        let mut dig = [0u8; 32];
        dig.copy_from_slice(&self.state_as_buf()[..32]);
        dig
    }
    pub fn extract_64(&mut self) -> [u8; 64] {
        assert_eq!(self.fill, 0);
        let mut dig = [0u8; 64];
        dig.copy_from_slice(&self.state_as_buf()[..64]);
        dig
    }
    pub fn squeeze_vec(&mut self, count: u32) -> Vec<u8> {
        let mut v = vec![0; count as usize];
        self.squeeze(&mut v);
        v
    }
    pub fn squeeze(&mut self, slice: &mut [u8]) {
        let mut ct = 0;
        loop {
            let buf = self.state_as_buf();
            let remaining = slice.len() - ct;
            if remaining < buf.len() {
                slice[ct..].copy_from_slice(&buf[..remaining]);
                self.fill += remaining as u8;
                break;
            }
            slice[ct..(ct + buf.len())].copy_from_slice(&buf);
            ct += buf.len();
            permute(&mut self.state, self.rounds);
            self.fill = 0;
        }
    }
}

pub fn sha3_256(m: &[u8]) -> [u8; 32] {
    let mut sp = Sponge::new_256();
    sp.ingest(m);
    sp.finish(suffix::SHA);
    sp.extract_32()
}
pub fn sha3_256_file(path: &str) -> [u8; 32] {
    use std::fs::read;
    let bytes = read(path).unwrap();
    sha3_256(&bytes)
}
pub fn sha3_512(m: &[u8]) -> [u8; 64] {
    let mut sp = Sponge::new_512();
    sp.ingest(m);
    sp.finish(suffix::SHA);
    sp.extract_64()
}

// S = m || c || len(c)
// |S| <= 8192 then
//   Node* = S || '11' -- 0x07
// |S| > 8192 then each CV_i = F[256](S_i || '110' -- 0x0B)
//   Node* = S_0 || '110..0'64 -- 0x03 00* || CV_0 .. || len(n-1) || FF FF || '01' -- 0x06
#[derive(Debug)]
pub struct K12 {
    pub byte_count: usize,
    pub trunk: Sponge,
    pub leaf: Sponge,
}
impl K12 {
    pub const SEG:  usize = 1 << 13;
    pub const MASK: usize = K12::SEG - 1;
    pub fn new() -> K12 {
        K12 { byte_count: 0, trunk: Sponge::new_k12(), leaf: Sponge::new_k12() }
    }
    fn leaf_ingest(&mut self, bytes: &[u8]) {
        let mut b = bytes;
        loop {
            let remaining = K12::SEG - (self.byte_count & K12::MASK);
            if remaining > b.len() {
                self.leaf.ingest(b);
                self.byte_count += b.len();
                break;
            }
            self.leaf.ingest(&b[..remaining]);
            self.byte_count += remaining;
            assert_eq!(self.byte_count & K12::MASK, 0);
            self.leaf.finish(suffix::K12_LEAF);
            self.trunk.ingest(&self.leaf.extract_32());
            self.leaf.reset();
            b = &b[remaining..];
        }
    }
    pub fn ingest(&mut self, bytes: &[u8]) {
        if self.byte_count < K12::SEG {
            let remaining = K12::SEG - self.byte_count;
            if remaining > bytes.len() {
                self.trunk.ingest(bytes);
                self.byte_count += bytes.len();
            } else {
                self.trunk.ingest(&bytes[..remaining]);
                self.byte_count += remaining;
                assert_eq!(self.byte_count, K12::SEG);
                self.trunk.ingest(&3u64.to_le_bytes());
                self.leaf_ingest(&bytes[remaining..]);
            }
        } else {
            self.leaf_ingest(bytes);
        }
    }
    pub fn finish(&mut self, custom: &[u8]) {
        self.ingest(custom);
        let (c_len, c_len_bytes) = length_encode(custom.len() as u32);
        self.ingest(&c_len[..(c_len_bytes - 1)]);
        let last_byte = c_len[c_len_bytes - 1];
        if self.byte_count < K12::SEG {
            self.trunk.ingest(&[last_byte]);
            self.byte_count += 1;
            self.trunk.finish(suffix::K12_ONE);
            return;
        }
        self.leaf.ingest(&[last_byte]);
        self.byte_count += 1;
        self.leaf.finish(suffix::K12_LEAF);
        self.trunk.ingest(&self.leaf.extract_32());
        let segment_ct = (self.byte_count + K12::MASK) >> 13;
        let (s_len, s_len_bytes) = length_encode(segment_ct as u32 - 1);
        self.trunk.ingest(&s_len[..s_len_bytes]);
        self.trunk.ingest(&[0xFF, 0xFF]);
        self.trunk.finish(suffix::K12_MANY);
    }
    pub fn squeeze_vec(&mut self, count: u32) -> Vec<u8> {
        self.trunk.squeeze_vec(count)
    }
    pub fn squeeze(&mut self, slice: &mut [u8]) {
        self.trunk.squeeze(slice);
    }
}

pub fn length_encode(x: u32) -> ([u8; 8], usize) {
    let mut x = x as u64;
    let zero_bytes = x.leading_zeros() >> 3;
    let sig_bytes = 8 - zero_bytes;
    x = (x << (1 << 3)) | sig_bytes as u64;
    x = x << ((zero_bytes - 1) << 3);
    (x.to_be_bytes(), sig_bytes as usize + 1)
}

pub fn k12(msg: &[u8], custom: &[u8], digest_len: u32) -> Vec<u8> {
    let mut k = K12::new();
    k.ingest(msg);
    k.finish(custom);
    k.squeeze_vec(digest_len)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_length_encode() {
        let (b, len) = length_encode(0);
        assert_eq!(b[..len], [0x00]);
        let (b, len) = length_encode(12);
        assert_eq!(b[..len], [0x0C, 0x01]);
        let (b, len) = length_encode(65538);
        assert_eq!(b[..len], [0x01, 0x00, 0x02, 0x03]);
    }
    #[test]
    fn test_sha() {
        assert_eq!(sha3_256(&[]),
                   [0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66,
                    0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6, 0x62,
                    0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa,
                    0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a]);
        assert_eq!(sha3_256(&[0xA3u8; 200]),
                   [0x79, 0xf3, 0x8a, 0xde, 0xc5, 0xc2, 0x03, 0x07,
                    0xa9, 0x8e, 0xf7, 0x6e, 0x83, 0x24, 0xaf, 0xbf,
                    0xd4, 0x6c, 0xfd, 0x81, 0xb2, 0x2e, 0x39, 0x73,
                    0xc6, 0x5f, 0xa1, 0xbd, 0x9d, 0xe3, 0x17, 0x87]);
        assert_eq!(sha3_256(b"abc"),
                   [0x3a, 0x98, 0x5d, 0xa7, 0x4f, 0xe2, 0x25, 0xb2,
                    0x04, 0x5c, 0x17, 0x2d, 0x6b, 0xd3, 0x90, 0xbd,
                    0x85, 0x5f, 0x08, 0x6e, 0x3e, 0x9d, 0x52, 0x5b,
                    0x46, 0xbf, 0xe2, 0x45, 0x11, 0x43, 0x15, 0x32]);
        assert_eq!(sha3_512(&[]),
                   [0xa6, 0x9f, 0x73, 0xcc, 0xa2, 0x3a, 0x9a, 0xc5,
                    0xc8, 0xb5, 0x67, 0xdc, 0x18, 0x5a, 0x75, 0x6e,
                    0x97, 0xc9, 0x82, 0x16, 0x4f, 0xe2, 0x58, 0x59,
                    0xe0, 0xd1, 0xdc, 0xc1, 0x47, 0x5c, 0x80, 0xa6,
                    0x15, 0xb2, 0x12, 0x3a, 0xf1, 0xf5, 0xf9, 0x4c,
                    0x11, 0xe3, 0xe9, 0x40, 0x2c, 0x3a, 0xc5, 0x58,
                    0xf5, 0x00, 0x19, 0x9d, 0x95, 0xb6, 0xd3, 0xe3,
                    0x01, 0x75, 0x85, 0x86, 0x28, 0x1d, 0xcd, 0x26]);
        assert_eq!(sha3_512(&[0xA3u8; 200]),
                   [0xe7, 0x6d, 0xfa, 0xd2, 0x20, 0x84, 0xa8, 0xb1,
                    0x46, 0x7f, 0xcf, 0x2f, 0xfa, 0x58, 0x36, 0x1b,
                    0xec, 0x76, 0x28, 0xed, 0xf5, 0xf3, 0xfd, 0xc0,
                    0xe4, 0x80, 0x5d, 0xc4, 0x8c, 0xae, 0xec, 0xa8,
                    0x1b, 0x7c, 0x13, 0xc3, 0x0a, 0xdf, 0x52, 0xa3,
                    0x65, 0x95, 0x84, 0x73, 0x9a, 0x2d, 0xf4, 0x6b,
                    0xe5, 0x89, 0xc5, 0x1c, 0xa1, 0xa4, 0xa8, 0x41,
                    0x6d, 0xf6, 0x54, 0x5a, 0x1c, 0xe8, 0xba, 0x00]);
        assert_eq!(sha3_512(b"abc"),
                   [0xb7, 0x51, 0x85, 0x0b, 0x1a, 0x57, 0x16, 0x8a,
                    0x56, 0x93, 0xcd, 0x92, 0x4b, 0x6b, 0x09, 0x6e,
                    0x08, 0xf6, 0x21, 0x82, 0x74, 0x44, 0xf7, 0x0d,
                    0x88, 0x4f, 0x5d, 0x02, 0x40, 0xd2, 0x71, 0x2e,
                    0x10, 0xe1, 0x16, 0xe9, 0x19, 0x2a, 0xf3, 0xc9,
                    0x1a, 0x7e, 0xc5, 0x76, 0x47, 0xe3, 0x93, 0x40,
                    0x57, 0x34, 0x0b, 0x4c, 0xf4, 0x08, 0xd5, 0xa5,
                    0x65, 0x92, 0xf8, 0x27, 0x4e, 0xec, 0x53, 0xf0]);
    }
    fn cyclic_pattern(len: usize) -> Vec<u8> {
        (0..=0xFA).cycle().take(len).collect()
    }
    #[test]
    fn test_k12() {
        assert_eq!(k12(&[], &[], 32),
                   [0x1a, 0xc2, 0xd4, 0x50, 0xfc, 0x3b, 0x42, 0x05,
                    0xd1, 0x9d, 0xa7, 0xbf, 0xca, 0x1b, 0x37, 0x51,
                    0x3c, 0x08, 0x03, 0x57, 0x7a, 0xc7, 0x16, 0x7f,
                    0x06, 0xfe, 0x2c, 0xe1, 0xf0, 0xef, 0x39, 0xe5]);
        assert_eq!(k12(&[], &[], 64),
                   [0x1a, 0xc2, 0xd4, 0x50, 0xfc, 0x3b, 0x42, 0x05,
                    0xd1, 0x9d, 0xa7, 0xbf, 0xca, 0x1b, 0x37, 0x51,
                    0x3c, 0x08, 0x03, 0x57, 0x7a, 0xc7, 0x16, 0x7f,
                    0x06, 0xfe, 0x2c, 0xe1, 0xf0, 0xef, 0x39, 0xe5,
                    0x42, 0x69, 0xc0, 0x56, 0xb8, 0xc8, 0x2e, 0x48,
                    0x27, 0x60, 0x38, 0xb6, 0xd2, 0x92, 0x96, 0x6c,
                    0xc0, 0x7a, 0x3d, 0x46, 0x45, 0x27, 0x2e, 0x31,
                    0xff, 0x38, 0x50, 0x81, 0x39, 0xeb, 0x0a, 0x71]);
        assert_eq!(k12(&[], &[], 10_032).split_off(10_000),
                   [0xe8, 0xdc, 0x56, 0x36, 0x42, 0xf7, 0x22, 0x8c,
                    0x84, 0x68, 0x4c, 0x89, 0x84, 0x05, 0xd3, 0xa8,
                    0x34, 0x79, 0x91, 0x58, 0xc0, 0x79, 0xb1, 0x28,
                    0x80, 0x27, 0x7a, 0x1d, 0x28, 0xe2, 0xff, 0x6d]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(0)), &[], 32),
                   [0x2b, 0xda, 0x92, 0x45, 0x0e, 0x8b, 0x14, 0x7f,
                    0x8a, 0x7c, 0xb6, 0x29, 0xe7, 0x84, 0xa0, 0x58,
                    0xef, 0xca, 0x7c, 0xf7, 0xd8, 0x21, 0x8e, 0x02,
                    0xd3, 0x45, 0xdf, 0xaa, 0x65, 0x24, 0x4a, 0x1f]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(1)), &[], 32),
                   [0x6b, 0xf7, 0x5f, 0xa2, 0x23, 0x91, 0x98, 0xdb,
                    0x47, 0x72, 0xe3, 0x64, 0x78, 0xf8, 0xe1, 0x9b,
                    0x0f, 0x37, 0x12, 0x05, 0xf6, 0xa9, 0xa9, 0x3a,
                    0x27, 0x3f, 0x51, 0xdf, 0x37, 0x12, 0x28, 0x88]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(2)), &[], 32),
                   [0x0c, 0x31, 0x5e, 0xbc, 0xde, 0xdb, 0xf6, 0x14,
                    0x26, 0xde, 0x7d, 0xcf, 0x8f, 0xb7, 0x25, 0xd1,
                    0xe7, 0x46, 0x75, 0xd7, 0xf5, 0x32, 0x7a, 0x50,
                    0x67, 0xf3, 0x67, 0xb1, 0x08, 0xec, 0xb6, 0x7c]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(3)), &[], 32),
                   [0xcb, 0x55, 0x2e, 0x2e, 0xc7, 0x7d, 0x99, 0x10,
                    0x70, 0x1d, 0x57, 0x8b, 0x45, 0x7d, 0xdf, 0x77,
                    0x2c, 0x12, 0xe3, 0x22, 0xe4, 0xee, 0x7f, 0xe4,
                    0x17, 0xf9, 0x2c, 0x75, 0x8f, 0x0d, 0x59, 0xd0]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(4)), &[], 32),
                   [0x87, 0x01, 0x04, 0x5e, 0x22, 0x20, 0x53, 0x45,
                    0xff, 0x4d, 0xda, 0x05, 0x55, 0x5c, 0xbb, 0x5c,
                    0x3a, 0xf1, 0xa7, 0x71, 0xc2, 0xb8, 0x9b, 0xae,
                    0xf3, 0x7d, 0xb4, 0x3d, 0x99, 0x98, 0xb9, 0xfe]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(5)), &[], 32),
                   [0x84, 0x4d, 0x61, 0x09, 0x33, 0xb1, 0xb9, 0x96,
                    0x3c, 0xbd, 0xeb, 0x5a, 0xe3, 0xb6, 0xb0, 0x5c,
                    0xc7, 0xcb, 0xd6, 0x7c, 0xee, 0xdf, 0x88, 0x3e,
                    0xb6, 0x78, 0xa0, 0xa8, 0xe0, 0x37, 0x16, 0x82]);
        assert_eq!(k12(&cyclic_pattern(17usize.pow(6)), &[], 32),
                   [0x3c, 0x39, 0x07, 0x82, 0xa8, 0xa4, 0xe8, 0x9f,
                    0xa6, 0x36, 0x7f, 0x72, 0xfe, 0xaa, 0xf1, 0x32,
                    0x55, 0xc8, 0xd9, 0x58, 0x78, 0x48, 0x1d, 0x3c,
                    0xd8, 0xce, 0x85, 0xf5, 0x8e, 0x88, 0x0a, 0xf8]);
        let ff = [0xff; 8];
        assert_eq!(k12(&ff[..0], &cyclic_pattern(41usize.pow(0)), 32),
                   [0xfa, 0xb6, 0x58, 0xdb, 0x63, 0xe9, 0x4a, 0x24,
                    0x61, 0x88, 0xbf, 0x7a, 0xf6, 0x9a, 0x13, 0x30,
                    0x45, 0xf4, 0x6e, 0xe9, 0x84, 0xc5, 0x6e, 0x3c,
                    0x33, 0x28, 0xca, 0xaf, 0x1a, 0xa1, 0xa5, 0x83]);
        assert_eq!(k12(&ff[..1], &cyclic_pattern(41usize.pow(1)), 32),
                   [0xd8, 0x48, 0xc5, 0x06, 0x8c, 0xed, 0x73, 0x6f,
                    0x44, 0x62, 0x15, 0x9b, 0x98, 0x67, 0xfd, 0x4c,
                    0x20, 0xb8, 0x08, 0xac, 0xc3, 0xd5, 0xbc, 0x48,
                    0xe0, 0xb0, 0x6b, 0xa0, 0xa3, 0x76, 0x2e, 0xc4]);
        assert_eq!(k12(&ff[..3], &cyclic_pattern(41usize.pow(2)), 32),
                   [0xc3, 0x89, 0xe5, 0x00, 0x9a, 0xe5, 0x71, 0x20,
                    0x85, 0x4c, 0x2e, 0x8c, 0x64, 0x67, 0x0a, 0xc0,
                    0x13, 0x58, 0xcf, 0x4c, 0x1b, 0xaf, 0x89, 0x44,
                    0x7a, 0x72, 0x42, 0x34, 0xdc, 0x7c, 0xed, 0x74]);
        assert_eq!(k12(&ff[..7], &cyclic_pattern(41usize.pow(3)), 32),
                   [0x75, 0xd2, 0xf8, 0x6a, 0x2e, 0x64, 0x45, 0x66,
                    0x72, 0x6b, 0x4f, 0xbc, 0xfc, 0x56, 0x57, 0xb9,
                    0xdb, 0xcf, 0x07, 0x0c, 0x7b, 0x0d, 0xca, 0x06,
                    0x45, 0x0a, 0xb2, 0x91, 0xd7, 0x44, 0x3b, 0xcf]);
    }
    // TODO test boundaries for K12: 8k +- 1, 16k +- 1
    //   test adding bytes one at a time, small chunks, big chunks. Shouldn't matter
}

