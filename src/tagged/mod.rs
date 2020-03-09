// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use handle::Handle;

pub mod guide;

pub static TAGGED_SENTINEL: u8 = 0;

pub struct Tagged { }

impl Tagged {
    pub fn new(sym: Handle, val: Handle) -> Unit {
        let needed = 1 /*prism*/ + 2;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Tagged>());
        prism.set(1, sym.unit());
        prism.set(2, val.unit());
        s.unit()
    }
}

impl Dispatch for Tagged {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        prism[1].handle().retire();
        prism[2].handle().retire();
        Segment::free(prism.segment())
    }
}

impl Identification for Tagged {
    fn type_name(&self) -> &'static str { "Tagged" }
    fn type_sentinel(&self) -> *const u8 { (& TAGGED_SENTINEL) as *const u8 }
}

impl Distinguish for Tagged {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let x = prism[1].handle().hash();
        let y = prism[2].handle().hash();
        x.wrapping_add(y)
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() && o.type_sentinel() == (& TAGGED_SENTINEL) as *const u8 {
            let oprism = o.prism();
            return prism[1].handle().eq(oprism[1].handle()) &&
                prism[2].handle().eq(oprism[2].handle())
        }
        false
    }
}

impl Aggregate for Tagged { }
impl Sequential for Tagged { }
impl Associative for Tagged { }
impl Reversible for Tagged { }
impl Sorted for Tagged { }

impl Notation for Tagged {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", prism[1].handle(), prism[2].handle())
    }
}

impl Numeral for Tagged { }
impl Callable for Tagged { }

#[cfg(test)]
mod tests {
    use super::*;
}

