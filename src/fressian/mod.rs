// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

pub mod reader;

pub fn something() {

}


pub struct Code {}
impl Code {
    pub const NEG_ONE: u8 = 0xFF;
    pub const U6:  (u8, u8, u8) = (0x00, 0x00, 0x40); // 64
    pub const I13: (u8, u8, u8) = (0x40, 0x50, 0x60); // 32
    pub const I20: (u8, u8, u8) = (0x60, 0x68, 0x70); // 16
    pub const I26: (u8, u8, u8) = (0x70, 0x72, 0x74); // 4
    pub const I34: (u8, u8, u8) = (0x74, 0x76, 0x78); // 4
    pub const I42: (u8, u8, u8) = (0x78, 0x7A, 0x7C); // 4
    pub const I50: (u8, u8, u8) = (0x7C, 0x7E, 0x80); // 4

    pub const CACHED_IMM: (u8, u8) = (0x80, 0xA0); // 32
    pub const STRUCT_IMM: (u8, u8) = (0xA0, 0xB0); // 16

    pub const LONGS: u8 = 0xB0;
    pub const DOUBLES: u8 = 0xB1;
    pub const BOOLS: u8 = 0xB2;
    pub const INTS: u8 = 0xB3;
    pub const FLOATS: u8 = 0xB4;
    pub const OBJECTS: u8 = 0xB5;

    pub const MAP: u8 = 0xC0;
    pub const SET: u8 = 0xC1;
    pub const UUID: u8 = 0xC3;
    pub const REGEX: u8 = 0xC4;
    pub const URI: u8 = 0xC5;
    pub const BIGINT: u8 = 0xC6;
    pub const BIGDEC: u8 = 0xC7;
    pub const INST: u8 = 0xC8;
    pub const SYMBOL: u8 = 0xC9;
    pub const KEYWORD: u8 = 0xCA;

    pub const CACHED: u8 = 0xCC;
    pub const CACHE: u8 = 0xCD;
    pub const CACHE_FOR_LATER: u8 = 0xCE;
    pub const FOOTER: u8 = 0xCF;

    pub const SMALL_BYTES: (u8, u8) = (0xD0, 0xD8); // 8
    pub const BYTES_CHUNK: u8 = 0xD8;
    pub const BYTES: u8 = 0xD9;
    pub const SMALL_STRING: (u8, u8) = (0xDA, 0xE2); // 8
    pub const STRING_CHUNK: u8 = 0xE2;
    pub const STRING: u8 = 0xE3;

    pub const SMALL_VEC: (u8, u8) = (0xE4, 0xEC); // 8
    pub const VEC: u8 = 0xEC;
    pub const LIST: u8 = 0xED;
    pub const UNBOUNDED_LIST: u8 = 0xEE;

    pub const EST_STRUCT: u8 = 0xEF;
    pub const STRUCT: u8 = 0xF0;
    pub const META: u8 = 0xF1;

    pub const ANY: u8 = 0xF4;
    pub const TRUE: u8 = 0xF5;
    pub const FALSE: u8 = 0xF6;
    pub const NIL: u8 = 0xF7;

    pub const I64: u8 = 0xF8;
    pub const F32: u8 = 0xF9;
    pub const F64: u8 = 0xFA;
    pub const F64_ZERO: u8 = 0xFB;
    pub const F64_ONE: u8 = 0xFC;

    pub const CLOSE_LIST: u8 = 0xFD;
    pub const RESET_CACHES: u8 = 0xFE;
}

pub const BYTE_CHUNK_SIZE: u32 = 1 << 16;


pub fn bit_width(x: i64) -> u32 {
    let y = if x.is_negative() { !x } else { x };
    (65 - y.leading_zeros()) // 1-64
}

