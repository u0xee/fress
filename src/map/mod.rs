// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::*;

use vector::guide::Guide;
pub mod pop;
use self::pop::Pop;
pub mod assoc;
//pub mod get;

pub const BITS: u32 = 4; // one of 5 (for 64 bit words) or 4 (for 32 bit words)
pub const ARITY: u32 = 1 << BITS;
pub const NODE_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;
pub const MAX_LEVELS: u32 = (32 + BITS - 1) / BITS;

pub static MAP_SENTINEL: u8 = 0;

pub struct Map {
    prism: Unit,
}

impl Map {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(11);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Map>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g
        };
        guide.root.set(0, Pop::new().unit());
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Map::new().value_unit().value()
    }
}

impl Dispatch for Map {
    fn tear_down(&self, prism: AnchoredLine) { unimplemented!() }
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
    fn get(&self, prism: AnchoredLine, k: Unit) -> Unit { unimplemented!() }
}
impl Sequential for Map {}
impl Associative for Map {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let h = k.value_unit().hash();
        // unalias root
        // no root entry? add to root
        // root key entry? Is it equal? We have an entry! Else a common prefix!
        // else child entry. Child, as anchored line to [pop, child segment] in unaliased segment
        // index_in_children, else index_in_keys, else keys below
        unimplemented!()
    }

    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        unimplemented!()
    }
}
impl Reversible for Map {}
impl Sorted for Map {}
impl Named for Map {}

impl Notation for Map {}

pub fn next_power(x: u32) -> u32 {
    (x + 1).next_power_of_two()
}

pub fn cap_at_arity_width(power: u32) -> u32 {
    power >> (power >> (BITS + 2))
}

/// Sizes a unit count to a power of two. With BITS as 5,
/// it returns 8, 16, 32, 64
pub fn size(unit_count: u32) -> u32 {
    cap_at_arity_width(next_power(unit_count | 0x4))
}

pub fn common_chunks(h1: u32, h2: u32) -> u32 {
    let top_chunks = (h1 ^ h2) >> BITS;
    let zeros = (top_chunks | 0x80000000u32).trailing_zeros();
    // compute this division with a faster algorithm?
    (zeros / BITS) + 1 /*for the bottom chunk*/
}

#[cfg(test)]
mod tests {
    use super::*;
}
