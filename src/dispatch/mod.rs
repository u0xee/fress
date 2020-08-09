// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
use value::Value;
use transduce::{Process};
pub mod mechanism;
use std::fmt;
use std::io;
use std::cmp;
use meta;

/// A trait to dynamically dispatch methods on heap values
pub trait Dispatch :
Identification +
Notation +
Distinguish +
Aggregate +
Sequential +
Associative +
Reversible +
Sorted +
Numeral +
Callable {
    /// Segment has alias-count of zero
    fn tear_down(&self, prism: AnchoredLine) {
        let seg = prism.segment();
        assert_eq!(0, seg.anchor().aliases());
        log!("Tearing down {} {}", self.type_name(), seg.unit().handle());
        Segment::free(seg);
    }
    fn alias_components(&self, prism: AnchoredLine) { return; }
    fn meta_value(&self, prism: AnchoredLine) -> Option<AnchoredLine> { None }
    fn logical_value(&self, prism: AnchoredLine) -> AnchoredLine { prism }
}
pub trait Identification {
    fn type_name(&self) -> &'static str;
}
pub trait Notation {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result { unimplemented!() }
    fn fressian(&self, prism:AnchoredLine, w: &mut dyn io::Write) -> io::Result<usize> { unimplemented!() }
}
pub trait Distinguish {
    fn hash(&self, prism: AnchoredLine) -> u32 { unimplemented!() }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool { unimplemented!() }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<cmp::Ordering> { unimplemented!() }
    // TODO Bitflags: agg? seq? map? set? num? Meta-nil Meta-true/false
    fn flags(&self, prism: AnchoredLine) -> u32 { unimplemented!() }
}
pub trait Aggregate {
    fn is_aggregate(&self, prism: AnchoredLine) -> bool { false }
    fn count(&self, prism: AnchoredLine) -> u32 { unimplemented!() }
    fn empty(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit { unimplemented!() }
    fn peek(&self, prism: AnchoredLine) -> *const Unit { unimplemented!() }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) { unimplemented!() }
    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit { unimplemented!() }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value { unimplemented!() }
}
pub trait Sequential {
    fn is_sequential(&self, prism: AnchoredLine) -> bool { false }
    // TODO return AnchoredLine instead
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit { unimplemented!() }
}
pub trait Associative {
    fn is_map(&self, prism: AnchoredLine) -> bool { false }
    fn is_set(&self, prism: AnchoredLine) -> bool { false }
    fn contains(&self, prism: AnchoredLine, x: Unit) -> bool { unimplemented!() }
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) { unimplemented!() }
    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit { unimplemented!() }
}
pub trait Reversible {
    fn reverse(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
}
pub trait Sorted {
    fn subrange(&self, prism: AnchoredLine, start: Unit, end: Unit) -> Unit { unimplemented!() }
}
pub trait Numeral {
    fn is_numeral(&self, prism: AnchoredLine) -> bool { false }
    fn inc(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn dec(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn add(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn subtract(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn neg(&self, prism: AnchoredLine) -> Unit { unimplemented!() }

    fn multiply(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn divide(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn remainder(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn modulus(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
}
pub trait Callable {
    fn invoke0(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn invoke1(&self, prism: AnchoredLine, a: Unit) -> Unit { unimplemented!() }
    fn invoke2(&self, prism: AnchoredLine, a: Unit, b: Unit) -> Unit { unimplemented!() }
    fn invoke3(&self, prism: AnchoredLine, a: Unit, b: Unit, c: Unit) -> Unit { unimplemented!() }
    // fn apply
}

// specific to integral?
pub trait Binary {
    fn bit_and(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn bit_or(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn bit_xor(&self, prism: AnchoredLine, other: Unit) -> Unit { unimplemented!() }
    fn bit_shl(&self, prism: AnchoredLine, shift: Unit) -> Unit { unimplemented!() }
    fn bit_shr(&self, prism: AnchoredLine, shift: Unit) -> Unit { unimplemented!() }
}

