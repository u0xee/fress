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

pub struct Set {
    prism: Unit,
}

impl Set {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(3 + map::size(1));
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Set>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
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
        map::tear_down::tear_down(prism, 0)
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        map::assoc::unaliased_root(Guide::hydrate(prism), 0).segment().unit()
    }
}

impl Identification for Set {
    fn type_name(&self) -> &'static str {
        "Set"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& SET_SENTINEL) as *const u8
    }
}

impl Distinguish for Set {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        // reduce over elements
        // sum hash codes. finalize
        unimplemented!()
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        // basic checks
        // compare structurally down tree
        // like tandem tear_down's
        unimplemented!()
    }
}

impl Aggregate for Set {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }

    fn empty(&self, prism: AnchoredLine) -> Unit {
        Set::new()
    }

    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        let h = k.handle().hash();
        if let Some(key_line) = map::get::get(prism, k, h, 0) {
            key_line.line().star()
        } else {
            (& handle::STATIC_NIL) as *const Unit
        }
    }

    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        let k = x;
        let h = k.handle().hash();
        let (g, key_slot) = map::assoc::assoc(prism, k, h, 0);
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
}

impl Sequential for Set {}

impl Associative for Set {
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

        write!(f, "#{{");
        let mut procs: [Box<Process>; 1] = [Box::new(Printer::new(f))];
        let _ = map::reduce::reduce(prism, &mut procs, 0);
        write!(f, "}}")
    }
}

impl Numeral for Set {}


#[cfg(test)]
mod tests {
    use super::*;
}
