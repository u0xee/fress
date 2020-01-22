// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::Value;
use handle::Handle;

// #![cfg(target_arch = "wasm32")]
// int128enc
// binary wasm

// each section = id byte_count bytes
// sections
// 1 types: vec(signatures)
//   signature =  0x60 vec(basic) vec(basic)

pub fn uleb128(x: u64, out: &mut [u8]) -> u32 {
    unimplemented!()
}

pub fn sleb128(x: u64, out: &mut [u8]) -> u32 {
    unimplemented!()
}

pub struct Op {}
impl Op {
    pub const TRAP: u8 = 0x00;
    pub const NOP: u8 = 0x01;
    pub const BLOCK: u8 = 0x02;
    pub const LOOP: u8 = 0x03;
    pub const IF: u8 = 0x04;
    pub const ELSE: u8 = 0x05;
    pub const END: u8 = 0x0B;
    pub const BR: u8 = 0x0C;
    pub const BR_IF: u8 = 0x0D;
    pub const BR_TAB: u8 = 0x0E;
    pub const RET: u8 = 0x0F;
    pub const CALL: u8 = 0x10;
    pub const CALL_X: u8 = 0x11;

    pub const DROP: u8 = 0x1A;
    pub const SELECT: u8 = 0x1B;

    pub const LOCAL: u8 = 0x20;
    pub const LOCAL_SET: u8 = 0x21;
    pub const LOCAL_TEE: u8 = 0x22;
    pub const GLOBAL: u8 = 0x23;
    pub const GLOBAL_SET: u8 = 0x24;

    pub const I32_LOAD: u8 = 0x28;
    pub const I32_STORE: u8 = 0x36;
    pub const MEM_PAGES: u8 = 0x3F;
    pub const MEM_GROW: u8 = 0x40;
    pub const I32_CONST: u8 = 0x41;
    pub const I32_EQZ: u8 = 0x45;
}
pub struct Type {}
impl Type {
    pub const I32: u8 = 0x7F;
    pub const I64: u8 = 0x7E;
    pub const F32: u8 = 0x7D;
    pub const F64: u8 = 0x7C;
    pub const VOID: u8 = 0x40;
    pub const FN: u8 = 0x60; //vec(args) vec(results)
    pub const MIN: u8 = 0x00;
    pub const MIN_MAX: u8 = 0x01;
    pub const FN_REF: u8 = 0x70;
    pub const GLOBAL_CONST: u8 = 0x00;
    pub const GLOBAL_MUT: u8 = 0x01;
}

pub struct Section {}
impl Section {
    pub const CUSTOM:  u8 = 0x00;
    pub const FN_TYPE: u8 = 0x01;
    pub const IMPORT:  u8 = 0x02;
    pub const FN_DEC:  u8 = 0x03;
    pub const TABLE:   u8 = 0x04;
    pub const MEM:     u8 = 0x05;
    pub const GLOBAL:  u8 = 0x06;
    pub const EXPORT:  u8 = 0x07;
    pub const START:   u8 = 0x08;
    pub const ELEM:    u8 = 0x09;
    pub const CODE:    u8 = 0x0A;
    pub const DATA:    u8 = 0x0B;
}
// vectors are u32 length, then elements
// names are vector of bytes (utf-8)

pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
pub const PAGE_SIZE: u32 = 1 << 16;

#[cfg(test)]
mod tests {
    use super::*;
}
