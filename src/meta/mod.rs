// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
//use value::Value;
use handle::Handle;

pub static META_SENTINEL: u8 = 0;

pub struct Meta { }

impl Meta {
    pub fn with_meta(v: Handle, m: Handle) -> Unit {
        if !v.is_ref() {
            unimplemented!()
        }
        let w = v.unaliased();
        Unit::zero()
    }

    /*
    pub fn x() -> Unit {
        let needed = 1 /*prism*/ + 2;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Meta>());
        prism.set(1, sym.unit());
        prism.set(2, val.unit());
        s.unit()
    }
    */
}

impl Dispatch for Meta {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        prism[1].handle().retire();
        prism[2].handle().retire();
        Segment::free(prism.segment())
    }
}

impl Identification for Meta {
    fn type_name(&self) -> &'static str { "Meta" }
    fn type_sentinel(&self) -> *const u8 { (& META_SENTINEL) as *const u8 }
}

//use std::cmp::Ordering;
impl Distinguish for Meta {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let x = prism[1].handle().hash();
        let y = prism[2].handle().hash();
        x.wrapping_add(y)
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() && o.type_sentinel() == (& META_SENTINEL) as *const u8 {
            let oprism = o.prism();
            return prism[1].handle().eq(oprism[1].handle()) &&
                prism[2].handle().eq(oprism[2].handle())
        }
        false
    }
}

impl Aggregate for Meta { }
impl Sequential for Meta { }
impl Associative for Meta { }
impl Reversible for Meta { }
impl Sorted for Meta { }

impl Notation for Meta {
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }

    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", prism[1].handle(), prism[2].handle())
    }
}

impl Numeral for Meta { }
impl Callable for Meta { }

#[cfg(test)]
mod tests {
    use super::*;
}

