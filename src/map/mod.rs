// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use Value;

use vector::guide::Guide;
pub mod pop;
use self::pop::Pop;
mod assoc;
mod get;

pub const BITS: u32 = 5; // one of 5 (for 64 bit words) or 4 (for 32 bit words)
pub const ARITY: u32 = 1 << BITS;
pub const NODE_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;
pub const MAX_LEVELS: u32 = (32 + BITS - 1) / BITS;

pub static MAP_SENTINEL: u8 = 0;

pub struct Map {
    prism: Unit,
}

impl Map {
    pub fn new() -> Value {
        let unit_count = 3 /* prism, guide, pop */ + 8 /* four pairs */;
        let mut s = Segment::new(unit_count);
        s[1] = prism::<Map>();
        s[2] = Guide::new().into();
        s[3] = Pop::new().into();
        Value { handle: Unit::from(s) }
    }

    fn line(&self) -> Line {
        Unit::from(&self.prism as *const Unit).into()
    }
}

impl Dispatch for Map {
    fn tear_down(&self) {
        unimplemented!()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Identification for Map {
    fn type_name(&self) -> String {
        "Map".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& MAP_SENTINEL) as *const u8
    }
}

impl Distinguish for Map {}

impl Aggregate for Map {
    fn get(&self, k: Unit) -> Unit {
        unimplemented!()
    }
}
impl Sequential for Map {}
impl Associative for Map {
    fn assoc(&self, k: Unit, v: Unit) -> (Unit, Unit) {
        unimplemented!()
    }
}
impl Reversible for Map {}
impl Sorted for Map {}
impl Named for Map {}

pub fn un_set(guide: Guide) -> u32 {
    let x: u64 = guide.post.into();
    (((x >> 52) & 1) ^ 1) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
}
