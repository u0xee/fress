// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Indexed array mapped trie, supporting vectors and lists.

use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use value::*;
use handle::Handle;
use transduce::Process;

pub mod guide;
use self::guide::Guide;
pub mod conj;
pub mod pop;
pub mod nth;
pub mod assoc;
pub mod eq;
pub mod tear_down;
pub mod reduce;
pub mod iter;
pub mod util;
use self::util::*;

/// Defines branching factor.
///
/// Can be 4, 5 or 6, making for sixteen, thirty-two or sixty-four way branching.
pub const BITS: u32 = 4; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

/// Vector dispatch.
pub struct Vector_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Vector_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_vector(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new() -> Unit {
    //log!("Vector new");
    let guide = {
        let cap = 1 /*prism*/ + Guide::units() + size(1);
        let s = Segment::new(cap);
        let prism = s.line_at(0);
        prism.set(0, prism_unit());
        let mut g = Guide::hydrate_top_bot(prism, 0, 0);
        g.is_compact_bit = 0x1;
        g
    };
    guide.store().segment().unit()
}
pub fn new_value() -> Value { new().handle().value() }

pub fn alias_components(prism: AnchoredLine) {
    let guide = Guide::hydrate(prism);
    if guide.count <= TAIL_CAP {
        guide.root.span(guide.count).split()
    } else {
        let root_count = root_content_count(tailoff(guide.count));
        let tail_and_roots = guide.root.offset(-1).span(root_count + 1);
        tail_and_roots.split()
    }
}
pub fn unaliased(prism: AnchoredLine) -> AnchoredLine {
    let seg = prism.segment();
    if seg.is_aliased() {
        if prism.index() == 0 {
            alias_components(prism);
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

impl Dispatch for Vector_ {
    fn tear_down(&self, prism: AnchoredLine) {
        //group!("Vector tear_down");
        tear_down::tear_down(prism);
        //group_end!();
    }
    fn alias_components(&self, prism: AnchoredLine) { alias_components(prism); }
}
impl Identification for Vector_ {
    fn type_name(&self) -> &'static str { "Vector" }
}
impl Distinguish for Vector_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        //group!("Vector hash");
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges(&mut self, _stack: &mut [Box<dyn Process>], v: &Value) -> Option<Value> {
                let h = v.hash() as u64;
                unsafe {
                    *self.ptr = cycle_abc(34, *self.ptr + h);
                }
                None
            }
            fn last_call(&mut self, _stack: &mut [Box<dyn Process>]) -> Value { Handle::nil().value() }
        }

        let mut y = cycle_abc(7, PI[321].wrapping_add(guide.count as u64));
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs);
        let h = cycle_abc(210, y) as u32;
        //log!("Hash of vector: {:#08X}", h);
        //group_end!();
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(v_prism) = find_prism(o) {
            //group!("Vector eq");
            let res = eq::eq(Guide::hydrate(prism), Guide::hydrate(v_prism));
            //group_end!();
            return res
        }
        use list;
        if let Some(l_prism) = list::find_prism(o) {
            let ct = {
                let ct = Guide::hydrate(prism).count;
                if Guide::hydrate(l_prism).count != ct {
                    return false
                }
                ct
            };
            for i in 0..ct {
                let x = nth::nth(prism, i)[0];
                let y = nth::nth(l_prism, ct - 1 - i)[0];
                if x.handle() != y.handle() { return false }
            }
            return true
        }
        false
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(v_prism) = find_prism(o) {
            let ct = Guide::hydrate(prism).count;
            let v_ct = Guide::hydrate(v_prism).count;
            let cmp_ct = ct.min(v_ct);
            for i in 0..cmp_ct {
                let x = nth::nth(prism, i)[0];
                let y = nth::nth(v_prism, i)[0];
                let res = x.handle().cmp(y.handle());
                match res {
                    Some(Ordering::Equal) => { },
                    _ => { return res },
                }
            }
            return Some(ct.cmp(&v_ct))
        }
        if o.is_ref() {
            let o_prism_unit = o.logical_value()[0];
            Some(prism_unit().cmp(&o_prism_unit))
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl Aggregate for Vector_ {
    fn is_aggregate(&self, prism: AnchoredLine) -> bool { true }
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, _prism: AnchoredLine) -> Unit { new() }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        //group!("Vector conj");
        let res = conj::conj(prism, x);
        //group_end!();
        res
    }
    fn peek(&self, prism: AnchoredLine) -> *const Unit {
        let guide = Guide::hydrate(prism);
        self.nth(prism, guide.count - 1)
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) { pop::pop(prism) }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        reduce::reduce(prism, process)
    }
}
impl Sequential for Vector_ {
    fn is_sequential(&self, prism: AnchoredLine) -> bool { true }
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        nth::nth(prism, idx).line().star()
    }
}
impl Associative for Vector_ {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let guide = Guide::hydrate(prism);
        let idx = k.handle().as_i64();
        if idx < 0 || (idx as u32) > guide.count {
            panic!("Index out of bounds: {} in vector of count {}", idx, guide.count);
        }
        k.handle().retire();
        assoc::assoc(prism, idx as u32, v)
    }
}
impl Reversible for Vector_ {}
impl Sorted for Vector_ {}
impl Notation for Vector_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        // conversion to and from &Formatter
        // factor out Printer parts
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
            fn inges(&mut self, _stack: &mut [Box<dyn Process>], v: &Value) -> Option<Value> {
                use std::mem::transmute;
                write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
                       "{}{}",
                       if self.is_first { self.is_first = false; "" } else { " " },
                       v).unwrap();
                None
            }
            fn last_call(&mut self, _stack: &mut [Box<dyn Process>]) -> Value {
                Handle::nil().value()
            }
        }

        write!(f, "[")?;
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Printer::new(f))];
        let _ = reduce::reduce(prism, &mut procs);
        write!(f, "]")
    }
}
impl Numeral for Vector_ {}
impl Callable for Vector_ {}

#[cfg(test)]
mod tests {
    use super::*;
}

