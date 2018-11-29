// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
pub mod mechanism;
use std::fmt::Display;
use std::cmp::Ordering;

/// A trait to dynamically dispatch methods on heap values
pub trait Dispatch :
Identification +
Distinguish +
Aggregate +
Sequential +
Associative +
Reversible +
Sorted +
Named {
    fn tear_down(&self, prism: AnchoredLine) { unimplemented!() }
    fn unaliased(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
}

pub trait Identification : Display {
    fn type_name(&self) -> String { unimplemented!() }
    fn type_sentinel(&self) -> *const u8 { unimplemented!() }
}

pub trait Distinguish {
    fn hash(&self, prism: AnchoredLine) -> u32 { unimplemented!() }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool { unimplemented!() }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Ordering { unimplemented!() }
}

pub trait Aggregate {
    fn count(&self, prism: AnchoredLine) -> u32 { unimplemented!() }
    fn empty(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit { unimplemented!() }
    fn meta(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit { unimplemented!() }
    fn peek(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) { unimplemented!() }
    fn get(&self, prism: AnchoredLine, k: Unit) -> Unit { unimplemented!() }
}

pub trait Sequential {
    fn nth(&self, prism: AnchoredLine, idx: u32) -> Unit { unimplemented!() }
}

pub trait Associative {
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

pub trait Named {
    fn name(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
    fn namespace(&self, prism: AnchoredLine) -> Unit { unimplemented!() }
}

