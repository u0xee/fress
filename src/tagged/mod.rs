// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use handle::Handle;

pub struct Tagged_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Tagged_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_tagged(h: Handle) -> bool { find_prism(h).is_some() }

// TODO hoist metadata from val?
pub fn new(sym: Handle, val: Handle) -> Unit {
    let needed = 3 /*prism sym val*/;
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, prism_unit());
    prism.set(1, sym.unit());
    prism.set(2, val.unit());
    s.unit()
}

impl Dispatch for Tagged_ {
    fn tear_down(&self, prism: AnchoredLine) {
        let seg = prism.segment();
        assert_eq!(0, seg.anchor().aliases());
        prism[1].handle().retire();
        prism[2].handle().retire();
        Segment::free(seg)
    }
    fn alias_components(&self, prism: AnchoredLine) {
        prism[1].handle().split();
        prism[2].handle().split();
    }
}
impl Identification for Tagged_ {
    fn type_name(&self) -> &'static str { "Tagged" }
}
impl Distinguish for Tagged_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let x = prism[1].handle().hash();
        let y = prism[2].handle().hash();
        x.wrapping_add(y)
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_tagged) = find_prism(o) {
            prism[1].handle().eq(o_tagged[1].handle()) &&
                prism[2].handle().eq(o_tagged[2].handle())
        } else {
            false
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_tagged) = find_prism(o) {
            let res = prism[1].handle().cmp(o_tagged[1].handle());
            return if let Some(Ordering::Equal) = res {
                prism[2].handle().cmp(o_tagged[2].handle())
            } else {
                res
            }
        }
        if o.is_ref() {
            let o_prism_unit = o.logical_value()[0];
            Some(prism_unit().cmp(&o_prism_unit))
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl Aggregate for Tagged_ { }
impl Sequential for Tagged_ { }
impl Associative for Tagged_ { }
impl Reversible for Tagged_ { }
impl Sorted for Tagged_ { }
impl Notation for Tagged_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", prism[1].handle(), prism[2].handle())
    }
}
impl Numeral for Tagged_ { }
impl Callable for Tagged_ { }

#[cfg(test)]
mod tests {
    use super::*;
}

