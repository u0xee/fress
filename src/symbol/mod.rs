// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::Value;
use handle::Handle;
use string::Str;
use string::guide::Guide;

pub static SYMBOL_SENTINEL: u8 = 0;

pub struct Symbol {
    prism: Unit,
}

impl Symbol {
    pub fn new(ns: &str, n: &str) -> Unit {
        let needed = 1 /*prism*/ + Guide::units() + 2;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Symbol>());
        let guide = Guide::hydrate_top_bot(prism, 0, 0);
        guide.root.set(0, Str::new_from_str(ns));
        guide.root.set(1, Str::new_from_str(n));
        guide.store().segment().unit()
    }

    pub fn new_name(n: &str) -> Unit {
        let needed = 1 /*prism*/ + Guide::units() + 2;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Symbol>());
        let guide = Guide::hydrate_top_bot(prism, 0, 0);
        guide.root.set(0, Handle::nil().unit);
        guide.root.set(1, Str::new_from_str(n));
        guide.store().segment().unit()
    }
}

impl Dispatch for Symbol {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        //Segment::free(prism.segment())
        unimplemented!()
    }
}

impl Identification for Symbol {
    fn type_name(&self) -> &'static str {
        "Symbol"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& SYMBOL_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Symbol {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        unimplemented!()
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        unimplemented!()
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        unimplemented!()
    }
}
// P G M? NS N
impl Aggregate for Symbol { }
impl Associative for Symbol { }
impl Sequential for Symbol { }
impl Reversible for Symbol { }
impl Sorted for Symbol { }
impl Notation for Symbol {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        let ns = guide.root[0].handle();
        let n = guide.root[1].handle();
        unimplemented!()
    }
}
impl Numeral for Symbol { }

#[cfg(test)]
mod tests {
    use super::*;
}

