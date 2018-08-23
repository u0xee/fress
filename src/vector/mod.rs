// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use Value;

pub mod guide;
use self::guide::Guide;
mod conj;

pub const BITS: u32 = 5; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

pub static VECTOR_SENTINEL: u8 = 0;

pub struct Vector {
    prism: Unit,
}

impl Vector {
    pub fn new() -> Value {
        let mut s = Segment::new(6);
        s[1] = prism::<Vector>();
        s[2] = Guide::new().into();
        Value { handle: Unit::from(s) }
    }

    fn line(&self) -> Line {
        Unit::from(&self.prism as *const Unit).into()
    }
}

impl Dispatch for Vector {
    fn tear_down(&self) {
        unimplemented!()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Identification for Vector {
    fn type_name(&self) -> String {
        "Vector".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }
}

impl Distinguish for Vector {}

impl Aggregate for Vector {
    fn conj(&self, x: Unit) -> Unit {
        conj::conj(self.line(), x)
    }
}

impl Sequential for Vector {}
impl Associative for Vector {}
impl Reversible for Vector {}
impl Sorted for Vector {}
impl Named for Vector {}



#[cfg(test)]
mod tests {
    use super::*;

}
