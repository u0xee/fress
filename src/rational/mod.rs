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
use Value;

pub static RATIONAL_SENTINEL: u8 = 0;

pub struct Rational {
    prism: Unit,
}

impl Rational {
    pub fn new(x: i64) -> Unit {
        let s = Segment::new(if cfg!(target_pointer_width = "32") { 3 } else { 2 });
        s.set(0, mechanism::prism::<Rational>());
        store(s.line_at(0), x);
        s.unit()
    }

    pub fn new_value(x: i64) -> Value {
        Rational::new(x).handle().value()
    }

    pub fn is_instance(h: Handle) -> bool {
        h.is_ref() && h.type_sentinel() == (& RATIONAL_SENTINEL) as *const u8
    }

    pub fn parse(negate: bool, top: &[u8], bot: &[u8]) -> Handle {
        unimplemented!()
    }
}

pub fn store(prism: AnchoredLine, x: i64) {
    if cfg!(target_pointer_width = "32") {
        prism.set(1, Unit::from(x as i32));
        prism.set(2, Unit::from((x >> 32) as i32));
    } else {
        prism.set(1, Unit::from(x));
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

impl Dispatch for Rational {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for Rational {
    fn type_name(&self) -> &'static str {
        "Rational"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& RATIONAL_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Rational {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        use hash::hash_64;
        let x = hydrate(prism) as u64;
        hash_64(x, 8)
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let c = self.cmp(prism, other);
        c.unwrap() == Ordering::Equal
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if !o.is_ref() {
            return Some(Ordering::Greater)
        }
        if o.type_sentinel() == (& RATIONAL_SENTINEL) as *const u8 {
            let x = hydrate(prism);
            let y = hydrate(o.prism());
            return Some(x.cmp(&y))
        }
        let ret = ((& RATIONAL_SENTINEL) as *const u8).cmp(&o.type_sentinel());
        Some(ret)
    }
}

impl Aggregate for Rational { }

impl Sequential for Rational { }

impl Associative for Rational { }

impl Reversible for Rational {}
impl Sorted for Rational {}

impl Notation for Rational {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let x = hydrate(prism);
        write!(f, "{}", x)
    }

    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let x = hydrate(prism);
        write!(f, "Rational[{}]", x)
    }
}

impl Numeral for Rational {
    fn inc(&self, prism: AnchoredLine) -> Unit {
        let x = hydrate(prism);
        let s = prism.segment();
        if s.is_aliased() {
            if s.unalias() == 0 {
                Segment::free(s);
            }
            Rational::new(x + 1)
        } else {
            store(prism, x + 1);
            s.unit()
        }
    }
    fn dec(&self, prism: AnchoredLine) -> Unit {
        let x = hydrate(prism);
        let s = prism.segment();
        if s.is_aliased() {
            if s.unalias() == 0 {
                Segment::free(s);
            }
            Rational::new(x - 1)
        } else {
            store(prism, x - 1);
            s.unit()
        }
    }
    fn neg(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn add(&self, prism: AnchoredLine, other: Unit) -> Unit {
        let o = other.handle();
        if Rational::is_instance(o) {
            let x = hydrate(prism);
            let y = hydrate(o.prism());
            let z = x + y;
            let s = prism.segment();
            if s.is_aliased() {
                if s.unalias() == 0 {
                    Segment::free(s);
                }
                let r = o.prism().segment();
                if r.is_aliased() {
                    if r.unalias() == 0 {
                        Segment::free(r);
                    }
                    Rational::new(z)
                } else {
                    store(o.prism(), z);
                    r.unit()
                }
            } else {
                store(prism, z);
                let r = o.prism().segment();
                if r.unalias() == 0 {
                    Segment::free(r);
                }
                s.unit()
            }
        } else {
            unimplemented!()
        }
    }
    fn subtract(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
    }
    fn multiply(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
    }
    fn divide(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
    }
    fn remainder(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
    }
    fn modulus(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
