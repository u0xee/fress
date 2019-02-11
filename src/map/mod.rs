// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Hash array mapped trie, supporting maps and sets.

use std::fmt;
use memory::*;
use dispatch::*;
use value::*;
use handle;
use handle::Handle;
use transducer::{Process};

use vector::guide::Guide;
pub mod pop;
use self::pop::Pop;
pub mod assoc;
use self::assoc::unaliased_root;
pub mod get;
pub mod reduce;
pub mod tear_down;
pub mod dissoc;

/// Defines branching factor.
///
/// Can be 4 or 5, making for sixteen way branching or thirty-two way branching.<br>
/// 32-bit platforms can only support sixteen way branching.
pub const BITS: u32 = 4; // one of 4 or 5
/// Tree arity, either 16 or 32.
pub const ARITY: u32 = 1 << BITS;
pub const MASK: u32 = ARITY - 1;
pub const MAX_LEVELS: u32 = (32 + BITS - 1) / BITS;

pub static MAP_SENTINEL: u8 = 0;

/// Map dispatch.
pub struct Map {
    prism: Unit,
}

impl Map {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(3 + size(1));
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Map>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g
        };
        guide.root.set(-1, Pop::new().unit());
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Map::new().handle().value()
    }
}

impl Dispatch for Map {
    fn tear_down(&self, prism: AnchoredLine) {
        tear_down::tear_down(prism, 1)
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unaliased_root(Guide::hydrate(prism), 1).segment().unit()
    }
}

impl Identification for Map {
    fn type_name(&self) -> &'static str {
        "Map"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& MAP_SENTINEL) as *const u8
    }
}

impl Distinguish for Map {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        // reduce over elements
        unimplemented!()
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        // basic checks
        // compare structurally down tree
        // like tandem tear_down's
        unimplemented!()
    }
}

impl Aggregate for Map {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        //println!("{:?}", Pop::from(guide.root[-1]));
        guide.count
    }

    fn empty(&self, prism: AnchoredLine) -> Unit {
        Map::new()
    }

    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        let h = k.handle().hash();
        if let Some(key_line) = get::get(prism, k, h, 1) {
            key_line.offset(1).line().star()
        } else {
            (& handle::STATIC_NIL) as *const Unit
        }
    }
}
impl Sequential for Map {}
impl Associative for Map {
    fn contains(&self, prism: AnchoredLine, k: Unit) -> bool {
        let h = k.handle().hash();
        get::get(prism, k, h, 1).is_some()
    }

    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let h = k.handle().hash();
        let (g, key_slot) = assoc::assoc(prism, k, h, 1);
        match key_slot {
            Ok(new_slot) => {
                new_slot.set(0, k);
                new_slot.set(1, v);
                (g.inc_count().store().segment().unit(), Handle::nil().unit())
            },
            Err(old_slot) => {
                k.handle().retire();
                let prev = old_slot[1];
                old_slot.set(1, v);
                (g.clear_hash().store().segment().unit(), prev)
            },
        }
    }

    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        let h = k.handle().hash();
        let g = dissoc::dissoc(prism, k, h, 1);
        g.segment().unit()
    }
}
impl Reversible for Map {}
impl Sorted for Map {}

// reduce, fold, into, iter, channels
// edn,fressian->reduce

impl Notation for Map {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let mut procs = {
            let mut procs: Vec<Box<Process>> = Vec::new();
            let b: Box<Process> = Box::new(Printer::new(f));
            procs.push(b);
            procs
        };
        let _ = reduce::reduce(prism, &mut procs, 1);
        write!(f, "")
    }
}

struct Printer {
    pub is_first: bool,
    pub f: usize,
}

impl Printer {
    pub fn new(f: &mut fmt::Formatter) -> Printer {
        use std::mem::transmute;
        unsafe { Printer { is_first: true, f: transmute::<& fmt::Formatter, usize>(f) } }
    }
}

impl Process for Printer {
    fn ingest_kv(&mut self, process_stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
        use std::mem::transmute;
        write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
               "{}{} {}",
               if self.is_first { self.is_first = false; "{" } else { ", " },
               k, v);
        None
    }
    fn last_call(&mut self, process_stack: &mut [Box<Process>]) -> Value {
        use std::mem::transmute;
        write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
               "}}");
        Handle::nil().value()
    }
}

impl Numeral for Map {}

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
