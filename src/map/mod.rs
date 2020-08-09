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
use transduce::{Process};

use vector::guide::Guide;
pub mod pop;
use self::pop::Pop;
pub mod assoc;
pub mod eq;
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

/// Map dispatch.
pub struct Map_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Map_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_map(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new() -> Unit {
    log!("map new");
    let guide = {
        let cap = 1 /*prism*/ + Guide::units() + 1 /*pop*/ + size(1);
        let s = Segment::new(cap);
        let prism = s.line_at(0);
        prism.set(0, prism_unit());
        let g = Guide::hydrate_top_bot(prism, 0, 0);
        g
    };
    guide.root.set(-1, Pop::new().unit());
    guide.store().segment().unit()
}
pub fn new_value() -> Value { new().handle().value() }

pub fn alias_components(prism: AnchoredLine, has_vals: u32) {
    let guide = Guide::hydrate(prism);
    let (child_count, key_count) = {
        let p = Pop::from(guide.root[-1]);
        (p.child_count() as i32, p.key_count())
    };
    for i in 0..child_count {
        guide.root[1 + (i << 1)].segment().alias();
    }
    let kvs = guide.root.offset(child_count << 1).span(key_count << has_vals);
    kvs.split();
}
pub fn unaliased(prism: AnchoredLine, has_vals: u32) -> AnchoredLine {
    let seg = prism.segment();
    if seg.is_aliased() {
        if prism.index() == 0 {
            alias_components(prism, has_vals);
        } else {
            seg.unit().handle()._alias_components();
        }
        let s = seg.carbon_copy();
        let p = prism.with_seg(s);
        seg.unit().handle().retire();
        p
    } else {
        prism
    }
}

impl Dispatch for Map_ {
    fn tear_down(&self, prism: AnchoredLine) {
        group!("Map tear down");
        tear_down::tear_down(prism, 1);
        group_end!();
    }
    fn alias_components(&self, prism: AnchoredLine) { alias_components(prism, 1); }
}
impl Identification for Map_ {
    fn type_name(&self) -> &'static str { "Map" }
}
impl Distinguish for Map_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        group!("Map hash");
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges_kv(&mut self, stack: &mut [Box<dyn Process>], k: &Value, v: &Value) -> Option<Value> {
                let kh = k.hash() as u64;
                let vh = v.hash() as u64;
                let h = cycle_abc(256, (kh << 32) | vh);
                unsafe {
                    *self.ptr = (*self.ptr).wrapping_add(h);
                }
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value { Handle::nil().value() }
        }

        let mut y = cycle_abc(58, PI[123] + guide.count as u64);
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs, 1);
        let h = cycle_abc(179, y) as u32;
        log!("Hash of map {:#08X}", h);
        group_end!();
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(m_prism) = find_prism(o) {
            group!("Map eq");
            let res = eq::eq(Guide::hydrate(prism), Guide::hydrate(m_prism), 1);
            group_end!();
            return res
        }
        // sorted_map
        // else, compare keys pairwise
        false
    }
}
impl Aggregate for Map_ {
    fn is_aggregate(&self, prism: AnchoredLine) -> bool { true }
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, prism: AnchoredLine) -> Unit { new() }
    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        let h = k.handle().hash();
        if let Some(key_line) = get::get(prism, k, h, 1) {
            key_line.offset(1).line().star()
        } else {
            (& handle::STATIC_NIL) as *const Unit
        }
    }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        reduce::reduce(prism, process, 1)
    }
}
impl Sequential for Map_ { }
impl Associative for Map_ {
    fn is_map(&self, prism: AnchoredLine) -> bool { true }
    fn contains(&self, prism: AnchoredLine, k: Unit) -> bool {
        let h = k.handle().hash();
        get::get(prism, k, h, 1).is_some()
    }
    // TODO assoc-in (recursive assoc)
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let h = k.handle().hash();
        group!("map assoc");
        let (g, key_slot) = assoc::assoc(prism, k, h, 1);
        group_end!();
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
impl Reversible for Map_ {}
impl Sorted for Map_ {}
impl Notation for Map_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO print in prefix map form
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
            fn inges_kv(&mut self, stack: &mut [Box<dyn Process>], k: &Value, v: &Value) -> Option<Value> {
                use std::mem::transmute;
                write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
                       "{}{} {}",
                       if self.is_first { self.is_first = false; "" } else { ", " },
                       k, v).unwrap();
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value {
                Handle::nil().value()
            }
        }

        write!(f, "{{")?;
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Printer::new(f))];
        let _ = reduce::reduce(prism, &mut procs, 1);
        write!(f, "}}")
    }
}
impl Numeral for Map_ {}
impl Callable for Map_ {}

pub fn next_power(x: u32) -> u32 { (x + 1).next_power_of_two() }
pub fn cap_at_arity_width(power: u32) -> u32 { power >> (power >> (BITS + 2)) }
/// Sizes a unit count to a power of two.
///
/// With BITS as 5, it returns 8, 16, 32, 64.
pub fn size(unit_count: u32) -> u32 { cap_at_arity_width(next_power(unit_count | 0x4)) }

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
