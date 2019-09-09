// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Sorted balanced tree, supporting maps and sets.

use std::fmt;
use memory::*;
use dispatch::*;
use value::*;
use handle::Handle;
use vector::guide::Guide;

/// Defines branching factor.
///
/// Can be 4 or 5, making for sixteen way branching or thirty-two way branching.<br>
/// 32-bit platforms can only support sixteen way branching.
pub const BITS: u32 = 4; // one of 4 or 5
/// Tree arity, either 16 or 32.
pub const ARITY: u32 = 1 << BITS;
pub const MASK: u32 = ARITY - 1;
pub const MAX_LEVELS: u32 = (32 + BITS - 1) / BITS;

pub static SORTED_MAP_SENTINEL: u8 = 0;

/// SortedMap dispatch.
pub struct SortedMap {
    prism: Unit,
}

impl SortedMap {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(3 + size(1));
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<SortedMap>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g
        };
        unimplemented!();
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        SortedMap::new().handle().value()
    }
}

impl Dispatch for SortedMap {
    fn tear_down(&self, prism: AnchoredLine) {
        unimplemented!()
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
}

impl Identification for SortedMap {
    fn type_name(&self) -> &'static str {
        "SortedMap"
    }

    fn type_sentinel(&self) -> *const u8 {
        (&SORTED_MAP_SENTINEL) as *const u8
    }
}

impl Distinguish for SortedMap {}

impl Aggregate for SortedMap {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }

    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        unimplemented!()
    }
}
impl Sequential for SortedMap {}
impl Associative for SortedMap {
    fn contains(&self, prism: AnchoredLine, k: Unit) -> bool {
        unimplemented!()
    }

    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        unimplemented!()
    }

    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        unimplemented!()
    }
}
impl Reversible for SortedMap {}
impl Sorted for SortedMap {}

// reduce, fold, into, iter, channels
// edn,fressian->reduce

impl Notation for SortedMap {}
impl Numeral for SortedMap {}

pub fn next_power(x: u32) -> u32 {
    (x + 1).next_power_of_two()
}

pub fn cap_at_arity_width(power: u32) -> u32 {
    power >> (power >> (BITS + 2))
}

/// Sizes a unit count to a power of two.
///
/// With BITS as 5, it returns 8, 16, 32, 64.
pub fn size(unit_count: u32) -> u32 {
    cap_at_arity_width(next_power(unit_count | 0x4))
}

pub fn common_chunks(h1: u32, h2: u32) -> u32 {
    let top_chunks = (h1 ^ h2) >> BITS;
    let zeros = (top_chunks | 0x80000000u32).trailing_zeros();
    divide_by_bits(zeros) + 1 /*for the bottom chunk*/
}

pub fn divide_by_five(x: u32) -> u32 {
    let p = x as u64 * 0x33333334u64;
    (p >> 32) as u32
}

pub fn divide_by_bits(x: u32) -> u32 {
    if BITS == 4 {
        x >> 2
    } else {
        divide_by_five(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}