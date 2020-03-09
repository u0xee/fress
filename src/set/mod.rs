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
use map;
use handle;
use handle::Handle;
use transduce::{Process};
use vector::guide::Guide;

pub static SET_SENTINEL: u8 = 0;

pub struct Set { }

// Set library:
// intersection, union, difference, symmetric difference
// disjoint?, subset?

impl Set {
    pub fn new() -> Unit {
        log!("set new");
        let guide = {
            let cap = 1 /*prism*/ + Guide::units() + 1 /*pop*/ + map::size(1);
            let s = Segment::new(cap);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Set>());
            let g = Guide::hydrate_top_bot(prism, 0, 0);
            g
        };
        guide.root.set(-1, map::pop::Pop::new().unit());
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Set::new().handle().value()
    }
}

impl Dispatch for Set {
    fn tear_down(&self, prism: AnchoredLine) {
        group!("set tear down");
        map::tear_down::tear_down(prism, 0);
        group_end!();
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        map::assoc::unaliased_root(Guide::hydrate(prism), 0).segment().unit()
    }
}

impl Identification for Set {
    fn type_name(&self) -> &'static str { "Set" }

    fn type_sentinel(&self) -> *const u8 { (& SET_SENTINEL) as *const u8 }
}

impl Distinguish for Set {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        group!("set hash");
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges(&mut self, stack: &mut [Box<dyn Process>], v: &Value) -> Option<Value> {
                let vh = v.hash() as u64;
                let h = cycle_abc(181, (vh << 32) | vh);
                unsafe {
                    *self.ptr = (*self.ptr).wrapping_add(h);
                }
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value {
                Handle::nil().value()
            }
        }

        let mut y = cycle_abc(97, PI[487] + guide.count as u64);
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = map::reduce::reduce(prism, &mut procs, 0);
        let h = cycle_abc(27, y) as u32;
        log!("hash of set {:#08X}", h);
        group_end!();
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() {
            let o_prism = o.logical_value();
            if prism[0] == o_prism[0] {
                group!("set eq");
                let res = map::eq::eq(Guide::hydrate(prism), Guide::hydrate(o_prism), 0);
                group_end!();
                res
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Aggregate for Set {
    fn is_aggregate(&self) -> bool { true }
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }

    fn empty(&self, prism: AnchoredLine) -> Unit {
        Set::new()
    }

    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        let k = x;
        let h = k.handle().hash();
        group!("set conj");
        let (g, key_slot) = map::assoc::assoc(prism, k, h, 0);
        group_end!();
        match key_slot {
            Ok(new_slot) => {
                new_slot.set(0, k);
                g.inc_count().store().segment().unit()
            },
            Err(old_slot) => {
                k.handle().retire();
                g.store().segment().unit()
            },
        }
    }

    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        let h = k.handle().hash();
        if let Some(key_line) = map::get::get(prism, k, h, 0) {
            key_line.line().star()
        } else {
            (& handle::STATIC_NIL) as *const Unit
        }
    }

    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        map::reduce::reduce(prism, process, 0)
    }
}

impl Sequential for Set {}

impl Associative for Set {
    fn is_set(&self) -> bool { true }
    fn contains(&self, prism: AnchoredLine, k: Unit) -> bool {
        let h = k.handle().hash();
        map::get::get(prism, k, h, 0).is_some()
    }

    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        let h = k.handle().hash();
        let g = map::dissoc::dissoc(prism, k, h, 0);
        g.segment().unit()
    }
}

impl Reversible for Set {}
impl Sorted for Set {}
impl Notation for Set {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
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
            fn inges(&mut self, stack: &mut [Box<dyn Process>], v: &Value) -> Option<Value> {
                use std::mem::transmute;
                write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
                       "{}{}",
                       if self.is_first { self.is_first = false; "" } else { " " },
                       v).unwrap();
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value {
                Handle::nil().value()
            }
        }

        write!(f, "#{{")?;
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Printer::new(f))];
        let _ = map::reduce::reduce(prism, &mut procs, 0);
        write!(f, "}}")
    }
}

impl Numeral for Set {}
impl Callable for Set {}


#[cfg(test)]
mod tests {
    use super::*;
}
