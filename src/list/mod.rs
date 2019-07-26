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
use handle::Handle;
use transduce::Process;

use vector;
use vector::{BITS, TAIL_CAP, MASK};
use vector::util::{tailoff, root_content_count, digit_count};
use vector::guide::Guide;
pub mod reduce;

pub static LIST_SENTINEL: u8 = 0;

pub struct List {
    prism: Unit,
}

impl List {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(6);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<List>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g.is_compact_bit = 0x1;
            g
        };
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        List::new().handle().value()
    }
}

impl Dispatch for List {
    fn tear_down(&self, prism: AnchoredLine) {
        vector::tear_down::tear_down(prism);
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        vector::conj::unaliased_root(Guide::hydrate(prism)).segment().unit()
    }
}

impl Identification for List {
    fn type_name(&self) -> &'static str { "List" }
    fn type_sentinel(&self) -> *const u8 { (& LIST_SENTINEL) as *const u8 }
}

impl Distinguish for List {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
                let h = v.hash() as u64;
                unsafe {
                    *self.ptr = cycle_abc(34, *self.ptr + h);
                }
                None
            }
            fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value {
                Handle::nil().value()
            }
        }

        let mut y = cycle_abc(7, PI[321] + guide.count as u64);
        let mut procs: [Box<Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs);
        let h = cycle_abc(210, y) as u32;
        guide.set_hash(h).store().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() {
            let o_prism = o.logical_value();
            if prism[0] == o_prism[0] {
                vector::eq::eq(Guide::hydrate(prism), Guide::hydrate(o_prism))
            } else {
                use vector::VECTOR_SENTINEL;
                let p = o_prism[0];
                if mechanism::as_dispatch(&p).type_sentinel() == (& VECTOR_SENTINEL) as *const u8 {
                    unimplemented!()
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
}

impl Aggregate for List {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        List::new()
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        vector::conj::conj(prism, x)
    }
    fn meta(&self, prism: AnchoredLine) -> *const Unit {
        vector::meta::meta(prism)
    }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit {
        vector::meta::with_meta(prism, m)
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        vector::pop::pop(prism)
    }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<Process>]) -> Value {
        reduce::reduce(prism, process)
    }
}

impl Sequential for List {
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        let guide = Guide::hydrate(prism);
        if idx >= guide.count {
            panic!("Index out of bounds: {} in list of count {}", idx, guide.count);
        }
        vector::nth::nth(prism, guide.count - 1 - idx).line().star()
    }
}

fn key_into_idx(k: Unit) -> u32 {
    // TODO need general conversion to int
    let i: u32 = k.into();
    i >> 4
}

impl Associative for List {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let idx: u32 = key_into_idx(k);
        let guide = Guide::hydrate(prism);
        if idx >= guide.count {
            panic!("Index out of bounds: {} in list of count {}", idx, guide.count);
        }
        vector::assoc::assoc(prism, guide.count - 1 - idx, v)
    }
}

impl Reversible for List {}
impl Sorted for List {}
impl Notation for List {
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List|");
        self.edn(prism, f);
        write!(f, "|")
    }
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
            fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
                use std::mem::transmute;
                write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
                       "{}{}",
                       if self.is_first { self.is_first = false; "" } else { " " },
                       v);
                None
            }
            fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value {
                Handle::nil().value()
            }
        }

        write!(f, "(");
        let mut procs: [Box<Process>; 1] = [Box::new(Printer::new(f))];
        let _ = reduce::reduce(prism, &mut procs);
        write!(f, ")")
    }
}

impl Numeral for List {}

#[cfg(test)]
mod tests {
    use super::*;

}
