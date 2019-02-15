// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std;
use std::fmt;
use memory::*;
use dispatch::*;

pub static STRING_SENTINEL: u8 = 0;

pub struct String {
    prism: Unit,
}

impl String {
    pub fn new() -> Unit {
        let s = Segment::new(8);
        s.set(0, mechanism::prism::<String>());
        s.set(1, Unit::from(0));
        s.unit()
    }
}

pub fn hydrate(prism: AnchoredLine) -> i64 {
    unimplemented!()
}

impl Dispatch for String {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for String {
    fn type_name(&self) -> &'static str {
        "String"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& STRING_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for String {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        use hash::hash_64;
        unimplemented!()
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        unimplemented!()
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if !o.is_ref() {
            return Some(Ordering::Greater)
        }
        if o.type_sentinel() == (& STRING_SENTINEL) as *const u8 {
            unimplemented!()
        }
        let ret = ((& STRING_SENTINEL) as *const u8).cmp(&o.type_sentinel());
        Some(ret)
    }
}

impl Aggregate for String { }
impl Sequential for String { }
impl Associative for String { }
impl Reversible for String {}
impl Sorted for String {}
impl Notation for String {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
impl Numeral for String {}

#[cfg(test)]
mod tests {
    use super::*;

}
