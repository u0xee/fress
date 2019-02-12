// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use random::PI;
pub mod keccak;


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

pub fn hash_raw_256(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64) {
    a = a.wrapping_add(PI[0]); b = b.wrapping_add(PI[1]);
    c = c.wrapping_add(PI[2]); d = d.wrapping_add(PI[3]);
    let (e, f, g, h) = mix(a, b, c, d);
    end(e, f, g, h)
}

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

