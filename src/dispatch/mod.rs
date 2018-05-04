// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

mod mechanism;
pub use self::mechanism::{distributor, as_dispatch_obj};
use memory::unit::Unit;

#[derive(Debug)]
pub struct Distributor {
    pub opaque_method_table_ptr: Unit,
}

/// A trait to dynamically dispatch methods on heap values
pub trait Dispatch :
Identification +
Distinguish +
AggregateAbstractions +
StreamlinedMethods {}

use std::fmt::Display;

pub trait Identification : Display {
    fn type_name(&self) -> String;

    fn type_sentinel(&self) -> *const u8;
}

use std::cmp::Ordering;

pub trait Distinguish {
    fn hash(&self) -> u32;

    fn eq(&self, other: &Dispatch) -> bool;

    fn cmp(&self, other: &Dispatch) -> Ordering;
}

use method_union::*;

pub trait AggregateAbstractions : Identification {
    fn seq_value(&self) -> &Seq {
        panic!("{} is NOT a SeqValue", self.type_name())
    }

    fn coll_value(&self) -> &Coll {
        panic!("{} is NOT a CollValue", self.type_name())
    }

    fn associative_value(&self) -> &Associative {
        panic!("{} is NOT an AssociativeValue", self.type_name())
    }

    fn sequential_value(&self) -> &Sequential {
        panic!("{} is NOT a SequentialValue", self.type_name())
    }

    fn sorted_value(&self) -> &Sorted {
        panic!("{} is NOT a SortedValue", self.type_name())
    }

    fn numeric_value(&self) -> bool {
        panic!("{} is NOT a NumericValue", self.type_name())
    }
}

use Value;

pub trait StreamlinedMethods : Identification {
    fn conj(&mut self, x: Value) -> Value {
        panic!("Can't conj onto a {}", self.type_name())
    }

    fn empty(&mut self) -> Value {
        panic!("Can't call empty on a {}", self.type_name())
    }

    fn first(&self) {
        panic!("Can't call first on a {}", self.type_name())
    }

    fn rest(&self) {
        panic!("Can't call rest on a {}", self.type_name())
    }

    fn count(&self) {
        panic!("Can't count a {}", self.type_name())
    }

    fn get(&self) {
        panic!("Can't call get on a {}", self.type_name())
    }

    fn nth(&self) {
        panic!("Can't call nth on a {}", self.type_name())
    }
}