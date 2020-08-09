// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Base for closures

use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use value::*;
use handle::Handle;
use transduce::Process;

pub static FUNC_SENTINEL: u8 = 0;

/// Func dispatch.
pub struct Func { }

// Func lifecycle
// Seg alloc, set prism
// Set meta-data, which arities
// Set closed values
// ...
// invoke!
// choose arity, calculate f-table index, invoke extern
// [method written by my compiler] receives aline and args
//

// Func prism, guide, IFn table base, closed-over values

#[link(wasm_import_module = "ifn")]
extern {
    fn func_invoke0(segment: u32, index: u32, table_idx: u32);
    fn func_invoke1(segment: u32, index: u32, arg0: u32, table_idx: u32);
    fn func_invoke2(segment: u32, index: u32, arg0: u32, arg1: u32, table_idx: u32);
}

/*
impl Func {
    pub fn new() -> Unit {
        log!("vector new");
        let guide = {
            let cap = 1 /*prism*/ + Guide::units() + size(1);
            let s = Segment::new(cap);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Vector>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g.is_compact_bit = 0x1;
            g
        };
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value { Vector::new().handle().value() }
}

impl Dispatch for Func {
    fn tear_down(&self, prism: AnchoredLine) {
        group!("vector tear down");
        tear_down::tear_down(prism);
        group_end!();
    }
    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unaliased_root(Guide::hydrate(prism)).segment().unit()
    }
}

impl Identification for Vector {
    fn type_name(&self) -> &'static str { "Vector" }
    fn type_sentinel(&self) -> *const u8 { (& VECTOR_SENTINEL) as *const u8 }
}

impl Distinguish for Vector {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        group!("vector hash");
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

        let mut y = cycle_abc(7, PI[321] + guide.count as u64);
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs);
        let h = cycle_abc(210, y) as u32;
        log!("hash of vector {:#08X}", h);
        group_end!();
        guide.set_hash(h).store_hash().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() {
            let o_prism = o.logical_value();
            if prism[0] == o_prism[0] {
                group!("vector eq");
                let res = eq::eq(Guide::hydrate(prism), Guide::hydrate(o_prism));
                group_end!();
                res
            } else {
                use list::LIST_SENTINEL;
                let p = o_prism[0];
                if mechanism::as_dispatch(&p).type_sentinel() == (& LIST_SENTINEL) as *const u8 {
                    unimplemented!()
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    fn cmp(&self, _prism: AnchoredLine, _other: Unit) -> Option<Ordering> {
        // cast other to vector, compare pairwise
        unimplemented!("Vector compare")
    }
}

impl Aggregate for Vector {
    fn is_aggregate(&self) -> bool { true }
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, _prism: AnchoredLine) -> Unit { Vector::new() }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        group!("vector conj");
        let res = conj::conj(prism, x);
        group_end!();
        res
    }
    fn meta(&self, prism: AnchoredLine) -> *const Unit { meta::meta(prism) }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit { meta::with_meta(prism, m) }
    fn peek(&self, prism: AnchoredLine) -> *const Unit {
        let guide = Guide::hydrate(prism);
        self.nth(prism, guide.count - 1)
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) { pop::pop(prism) }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        reduce::reduce(prism, process)
    }
}

impl Sequential for Vector {
    fn is_sequential(&self) -> bool { true }
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        nth::nth(prism, idx).line().star()
    }
}

impl Associative for Vector {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let guide = Guide::hydrate(prism);
        let idx = k.handle().as_i64();
        k.handle().retire();
        if idx < 0 || (idx as u32) >= guide.count {
            panic!("Index out of bounds: {} in vector of count {}", idx, guide.count);
        }
        assoc::assoc(prism, idx as u32, v)
    }
}

impl Reversible for Vector {}
impl Sorted for Vector {}
impl Notation for Vector {
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

impl Numeral for Vector {}
impl Callable for Vector {}

#[cfg(test)]
mod tests {
    use super::*;

}

*/

