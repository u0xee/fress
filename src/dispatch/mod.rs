// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

mod mechanism;
pub use self::mechanism::prism;
mod value_unit;
pub use self::value_unit::ValueUnit;
use memory::unit::Unit;
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
    fn tear_down(&self) {
        unimplemented!()
    }
}

pub trait Identification : Display {
    fn type_name(&self) -> String {
        unimplemented!()
    }
    fn type_sentinel(&self) -> *const u8 {
        unimplemented!()
    }
}

pub trait Distinguish {
    fn hash(&self) -> u32 {
        unimplemented!()
    }
    fn eq(&self, other: Unit) -> bool {
        unimplemented!()
    }
    fn cmp(&self, other: Unit) -> Ordering {
        unimplemented!()
    }
}

pub trait Aggregate {
    fn count(&self) -> u32 {
        unimplemented!()
    }
    fn empty(&self) -> Unit {
        unimplemented!()
    }
    fn conj(&self, x: Unit) -> Unit {
        unimplemented!()
    }
    fn meta(&self) -> Unit {
        unimplemented!()
    }
    fn with_meta(&self, m: Unit) -> Unit {
        unimplemented!()
    }
    fn peek(&self) -> Unit {
        unimplemented!()
    }
    fn pop(&self) -> Unit {
        unimplemented!()
    }
    fn get(&self, k: Unit) -> Unit {
        unimplemented!()
    }
}

pub trait Sequential {
    fn nth(&self, idx: u32) -> Unit {
        unimplemented!()
    }
}

pub trait Associative {
    fn contains(&self, x: Unit) -> bool {
        unimplemented!()
    }
    fn assoc(&self, k: Unit, v: Unit) -> Unit {
        unimplemented!()
    }
    fn dissoc(&self, k: Unit) -> Unit {
        unimplemented!()
    }
}

pub trait Reversible {
    fn reverse(&self) -> Unit {
        unimplemented!()
    }
}

pub trait Sorted {
    fn subrange(&self, start: Unit, end: Unit) -> Unit {
        unimplemented!()
    }
}

pub trait Named {
    fn name(&self) -> Unit {
        unimplemented!()
    }
    fn namespace(&self) -> Unit {
        unimplemented!()
    }
}

