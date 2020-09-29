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
use std::cmp::Ordering;

use vector;
use vector::{BITS, TAIL_CAP, MASK};
use vector::util::{tailoff, root_content_count, digit_count, size};
use vector::guide::Guide;
pub mod reduce;

pub struct List_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<List_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_list(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new() -> Unit {
    //log!("List new");
    let s = vector::new().segment();
    s.set(0, prism_unit());
    s.unit()
}
pub fn new_value() -> Value { new().handle().value() }

impl Dispatch for List_ {
    fn tear_down(&self, prism: AnchoredLine) {
        //group!("List tear down");
        vector::tear_down::tear_down(prism);
        //group_end!();
    }
    fn alias_components(&self, prism: AnchoredLine) { vector::alias_components(prism); }
}
impl Identification for List_ {
    fn type_name(&self) -> &'static str { "List" }
}
impl Distinguish for List_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        //group!("List hash");
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges(&mut self, stack: &mut [Box<dyn Process>], v: &Value) -> Option<Value> {
                let h = v.hash() as u64;
                unsafe {
                    *self.ptr = cycle_abc(34, *self.ptr + h);
                }
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value {
                Handle::nil().value()
            }
        }

        let mut y = cycle_abc(7, PI[321].wrapping_add(guide.count as u64));
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs);
        let h = cycle_abc(210, y) as u32;
        //log!("Hash of list: {:#08X}", h);
        //group_end!();
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(l_prism) = find_prism(o) {
            //group!("List eq");
            let res = vector::eq::eq(Guide::hydrate(prism), Guide::hydrate(l_prism));
            //group_end!();
            return res
        }
        if vector::is_vector(o) {
            return o.eq(prism.segment().unit().handle())
        }
        false
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        unimplemented!()
    }
}
impl Aggregate for List_ {
    fn is_aggregate(&self, prism: AnchoredLine) -> bool { true }
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, prism: AnchoredLine) -> Unit { new() }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        //group!("List conj");
        let res = vector::conj::conj(prism, x);
        //group_end!();
        res
    }
    fn peek(&self, prism: AnchoredLine) -> *const Unit { self.nth(prism, 0) }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) { vector::pop::pop(prism) }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        reduce::reduce(prism, process)
    }
}
impl Sequential for List_ {
    fn is_sequential(&self, prism: AnchoredLine) -> bool { true }
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        let guide = Guide::hydrate(prism);
        if idx >= guide.count {
            panic!("Index out of bounds: {} in list of count {}", idx, guide.count);
        }
        vector::nth::nth(prism, guide.count - 1 - idx).line().star()
    }
}
impl Associative for List_ {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let guide = Guide::hydrate(prism);
        let idx = k.handle().as_i64();
        k.handle().retire();
        if idx < 0 || (idx as u32) > guide.count {
            panic!("Index out of bounds: {} in list of count {}", idx, guide.count);
        }
        vector::assoc::assoc(prism, guide.count - 1 - (idx as u32), v)
    }
}
impl Reversible for List_ { }
impl Sorted for List_ { }
impl Notation for List_ {
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

        write!(f, "(")?;
        let mut procs: [Box<dyn Process>; 1] = [Box::new(Printer::new(f))];
        let _ = reduce::reduce(prism, &mut procs);
        write!(f, ")")
    }
}
impl Numeral for List_ {}
impl Callable for List_ {}

#[cfg(test)]
mod tests {
    use super::*;

}
