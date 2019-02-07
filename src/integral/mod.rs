// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;

pub static INTEGRAL_SENTINEL: u8 = 0;

pub struct Integral {
    prism: Unit,
}

impl Integral {
    pub fn new(x: i64) -> Unit {
        let s = Segment::new(if cfg!(target_pointer_width = "32") { 3 } else { 2 });
        s.set(0, mechanism::prism::<Integral>());
        if cfg!(target_pointer_width = "32") {
            s.set(1, Unit::from(x as i32));
            s.set(2, Unit::from((x >> 32) as i32));
        } else {
            s.set(1, Unit::from(x));
        }
        s.unit()
    }
}

pub fn hydrate(prism: AnchoredLine) -> i64 {
    if cfg!(target_pointer_width = "32") {
        let low: u32 = prism[1].into();
        let hi: u32 = prism[2].into();
        let res = ((hi as u64) << 32) | (low as u64);
        res as i64
    } else {
        prism[1].into()
    }
}

impl Dispatch for Integral {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for Integral {
    fn type_name(&self) -> String {
        "Integral".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& INTEGRAL_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Integral {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        use hash::hash_64;
        let x = hydrate(prism) as u64;
        hash_64(x, 8)
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let c = self.cmp(prism, other);
        c == Ordering::Equal
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Ordering {
        let o = other.handle();
        if !o.is_ref() {
            return Ordering::Greater
        }
        if o.type_sentinel() == (& INTEGRAL_SENTINEL) as *const u8 {
            let x = hydrate(prism);
            let y = hydrate(other.handle().prism());
            return x.cmp(&y)
        }
        ((& INTEGRAL_SENTINEL) as *const u8).cmp(&o.type_sentinel())
    }
}

impl Aggregate for Integral { }

impl Sequential for Integral { }

impl Associative for Integral { }

impl Reversible for Integral {}
impl Sorted for Integral {}

impl Notation for Integral {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let x = hydrate(prism);
        write!(f, "{}", x)
    }

    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let x = hydrate(prism);
        write!(f, "Integral[{}]", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
