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

pub struct FloatPoint_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<FloatPoint_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_float(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new(x: f64) -> Unit {
    let guide = blank();
    store(guide.root, x);
    guide.store().segment().unit()
}

pub fn blank() -> Guide {
    let needed = 1 /*prism*/ + Guide::units() + if cfg!(target_pointer_width = "32") { 2 } else { 1 };
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, prism_unit());
    let guide = Guide::new(prism);
    guide
}

pub fn parse(negate: bool, whole: &[u8], part: &[u8], promote: bool) -> Handle {
    use std::str::from_utf8;
    let b = format!("{}.{}", from_utf8(whole).unwrap(), from_utf8(part).unwrap());
    if promote { unimplemented!() }
    let mut x = b.parse::<f64>().unwrap();
    if negate { x = -x; }
    let guide = {
        let g = blank();
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
    if promote { unimplemented!() }
    let mut x = b.parse::<f64>().unwrap();
    if negate { x = -x; }
    let guide = {
        let g = blank();
        if promote { g.set_big() } else { g }
    };
    store(guide.root, x);
    guide.store().segment().unit().handle()
}

pub fn inf() -> Handle {
    use std::f64::INFINITY;
    new(INFINITY).handle()
}
pub fn neg_inf() -> Handle {
    use std::f64::NEG_INFINITY;
    new(NEG_INFINITY).handle()
}
pub fn not_a_number() -> Handle {
    use std::f64::NAN;
    new(NAN).handle()
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

impl Dispatch for FloatPoint_ { }
impl Identification for FloatPoint_ {
    fn type_name(&self) -> &'static str { "FloatPoint" }
}
impl Distinguish for FloatPoint_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use hash::hash_64;
            let x = hydrate(guide.root);
            use memory::unit::f64_into_u64;
            hash_64(f64_into_u64(x), 8)
        };
        log!("Hash float point {} {:#08X}", prism.segment().unit().handle(), h);
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_float) = find_prism(o) {
            log!("FloatPoint eq");
            let guide = Guide::hydrate(prism);
            let guide2 = Guide::hydrate(o_float);
            let x = hydrate(guide.root);
            let y = hydrate(guide2.root);
            return x == y
        }
        false
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_float) = find_prism(o) {
            log!("FloatPoint cmp");
            let guide = Guide::hydrate(prism);
            let guide2 = Guide::hydrate(o_float);
            let x = hydrate(guide.root);
            let y = hydrate(guide2.root);
            return x.partial_cmp(&y)
        }
        if o.is_ref() {
            let o_prism_unit = o.logical_value()[0];
            Some(prism_unit().cmp(&o_prism_unit))
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl Aggregate for FloatPoint_ { }
impl Sequential for FloatPoint_ { }
impl Associative for FloatPoint_ { }
impl Reversible for FloatPoint_ {}
impl Sorted for FloatPoint_ {}
impl Notation for FloatPoint_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        let x = hydrate(guide.root);
        if x.is_finite() {
            if x.floor() == x {
                write!(f, "{}.0", x)
            } else {
                write!(f, "{}", x)
            }
        } else if x.is_nan() {
            write!(f, "##NaN")
        } else if x.is_sign_positive() {
            write!(f, "##Inf")
        } else {
            write!(f, "##-Inf")
        }
    }
}
impl Numeral for FloatPoint_ { }
impl Callable for FloatPoint_ { }

#[cfg(test)]
mod tests {
    use super::*;

}
