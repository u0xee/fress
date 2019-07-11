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

use integral::guide::Guide;

// Numbers. immediate i60 (28), f60 (28). boxed integral, rational, float point.
// Layout: i60I f60F, [prism guide{chunk_count} contents]
// methods (on guide?) to get/set chunks by index (32/64, LE/BE)
// inc dec, + -, * /, % mod, neg
// zero? neg? pos?, type tests, modular exponentiation,

// inc dec -> if(int) _ else fn_call to dispatch here
// neg if imm
// + - * / if both imm (x & y & 0x1) == 0x1 then _ else dispatch here
// zero? neg? pos? -> special case imm


pub static FLOATPOINT_SENTINEL: u8 = 0;

pub struct FloatPoint {
    prism: Unit,
}

impl FloatPoint {
    // TODO -0.0 read as 0.0
    pub fn new(x: f64) -> Unit {
        let guide = FloatPoint::blank();
        store(guide.root, x);
        guide.store().segment().unit()
    }

    pub fn blank() -> Guide {
        let needed = 1 /*prism*/ + Guide::units() + if cfg!(target_pointer_width = "32") { 2 } else { 1 };
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<FloatPoint>());
        let guide = Guide::new(prism);
        guide
    }

    pub fn is_instance(h: Handle) -> bool {
        h.is_ref() && h.type_sentinel() == (& FLOATPOINT_SENTINEL) as *const u8
    }

    pub fn parse(negate: bool, whole: &[u8], part: &[u8], promote: bool) -> Handle {
        use std::str::from_utf8;
        let b = format!("{}.{}", from_utf8(whole).unwrap(), from_utf8(part).unwrap());
        let mut x = b.parse::<f64>().unwrap();
        if negate { x = -x; }
        let guide = {
            let g = FloatPoint::blank();
            if promote { g.set_big() } else { g }
        };
        store(guide.root, x);
        guide.store().segment().unit().handle()
    }

    pub fn parse_exp(negate: bool, whole: &[u8], part: &[u8],
                     exp_negate: bool, exp: &[u8], promote: bool) -> Handle {
        use std::str::from_utf8;
        let b = format!("{}.{}e{}{}", from_utf8(whole).unwrap(), from_utf8(part).unwrap(),
                        if exp_negate { "-" } else { "" }, from_utf8(exp).unwrap());
        let mut x = b.parse::<f64>().unwrap();
        if negate { x = -x; }
        let guide = {
            let g = FloatPoint::blank();
            if promote { g.set_big() } else { g }
        };
        store(guide.root, x);
        guide.store().segment().unit().handle()
    }

    pub fn inf() -> Handle {
        use std::f64::INFINITY;
        FloatPoint::new(INFINITY).handle()
    }

    pub fn neg_inf() -> Handle {
        use std::f64::NEG_INFINITY;
        FloatPoint::new(NEG_INFINITY).handle()
    }

    pub fn not_a_number() -> Handle {
        use std::f64::NAN;
        FloatPoint::new(NAN).handle()
    }
}

pub fn store(line: AnchoredLine, x: f64) {
    use memory::unit::f64_into_u64;
    let x = f64_into_u64(x);
    if cfg!(target_pointer_width = "32") {
        line.set(0, Unit::from(x as u32));
        line.set(1, Unit::from((x >> 32) as u32));
    } else {
        line.set(0, Unit::from(x));
    }
}

pub fn hydrate(line: AnchoredLine) -> f64 {
    let x = if cfg!(target_pointer_width = "32") {
        let low: u32 = line[0].into();
        let hi:  u32 = line[1].into();
        ((hi as u64) << 32) | (low as u64)
    } else {
        line[0].u64()
    };
    use memory::unit::f64_from_u64;
    f64_from_u64(x)
}

impl Dispatch for FloatPoint {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for FloatPoint {
    fn type_name(&self) -> &'static str {
        "FloatPoint"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& FLOATPOINT_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for FloatPoint {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use hash::hash_64;
            let x = hydrate(guide.root);
            use memory::unit::f64_into_u64;
            hash_64(f64_into_u64(x), 8)
        };
        guide.set_hash(h).store().hash
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
        if o.type_sentinel() == (& FLOATPOINT_SENTINEL) as *const u8 {
            let guide = Guide::hydrate(prism);
            let guide2 = Guide::hydrate(o.prism());
            let x = hydrate(guide.root);
            let y = hydrate(guide2.root);
            return x.partial_cmp(&y)
        }
        let ret = ((& FLOATPOINT_SENTINEL) as *const u8).cmp(&o.type_sentinel());
        Some(ret)
    }
}

impl Aggregate for FloatPoint { }
impl Sequential for FloatPoint { }
impl Associative for FloatPoint { }
impl Reversible for FloatPoint {}
impl Sorted for FloatPoint {}

impl Notation for FloatPoint {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        let x = hydrate(guide.root);
        if x.is_finite() {
            // TODO print 4. as 4.0
            write!(f, "{}", x)
        } else if x.is_nan() {
            write!(f, "##NaN")
        } else if x.is_positive() {
            write!(f, "##Inf")
        } else {
            write!(f, "##-Inf")
        }
    }

    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatPoint[");
        self.edn(prism, f);
        write!(f, "]")
    }
}

impl Numeral for FloatPoint {
    fn inc(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn dec(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn neg(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn add(&self, prism: AnchoredLine, other: Unit) -> Unit {
        unimplemented!()
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
