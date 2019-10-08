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

pub mod guide;
use self::guide::Guide;

// Numbers. immediate i60 (28), f60 (28). boxed integral, rational, float point.
// Layout: i60I f60F, [prism guide{chunk_count} contents]
// methods (on guide?) to get/set chunks by index (32/64, LE/BE)
// inc dec, + -, * /, % mod, neg
// zero? neg? pos?, type tests, modular exponentiation,

// inc dec -> if(int) _ else fn_call to dispatch here
// neg if imm
// + - * / if both imm (x & y & 0x1) == 0x1 then _ else dispatch here
// zero? neg? pos? -> special case imm


pub static INTEGRAL_SENTINEL: u8 = 0;

pub struct Integral {
    prism: Unit,
}

impl Integral {
    pub fn new(x: i64) -> Unit {
        let guide = Integral::blank();
        store(guide.root, x);
        guide.store().segment().unit()
    }

    pub fn blank() -> Guide {
        let needed = 1 /*prism*/ + Guide::units() + if cfg!(target_pointer_width = "32") { 2 } else { 1 };
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Integral>());
        let guide = Guide::new(prism);
        guide
    }

    pub fn new_value(x: i64) -> Value {
        Integral::new(x).handle().value()
    }

    pub fn is_instance(h: Handle) -> bool {
        h.is_ref() && h.type_sentinel() == (& INTEGRAL_SENTINEL) as *const u8
    }

    pub fn parse(negate: bool, m: &[u8], promote: bool) -> Handle {
        if promote {
            return big_int(negate, m)
        }
        let mut x = 0i64;
        for b in m.iter() {
            if *b == b'_' {
                continue
            }
            x = x * 10 + (*b - b'0') as i64;
        }
        if negate { x = -x; }
        let guide = {
            let g = Integral::blank();
            if promote { g.set_big() } else { g }
        };
        store(guide.root, x);
        guide.store().segment().unit().handle()
    }

    pub fn parse_hex(negate: bool, m: &[u8], promote: bool) -> Handle {
        if promote { unimplemented!() }
        let mut x = 0i64;
        for b in m.iter() {
            if *b == b'_' {
                continue
            }
            let d = if *b <= b'9' { *b - b'0' }
                else if *b <= b'F' { *b - b'A' + 10 }
                    else { *b - b'a' + 10 };
            assert!(0 <= d && d < 16);
            x = (x << 4) + d as i64;
        }
        if negate { x = -x; }
        let guide = {
            let g = Integral::blank();
            if promote { g.set_big() } else { g }
        };
        store(guide.root, x);
        guide.store().segment().unit().handle()
    }

    pub fn parse_radix(negate: bool, radix: u32, m: &[u8]) -> Option<Handle> {
        let mut x = 0i64;
        for b in m.iter() {
            if *b == b'_' {
                continue
            }
            let d = if *b <= b'9' { *b - b'0' }
                else if *b <= b'Z' { *b - b'A' + 10 }
                    else { *b - b'a' + 10 };
            if d >= radix as u8 { return None }
            x = x * radix as i64 + d as i64;
        }
        if negate { x = -x; }
        Some(Integral::new(x).handle())
    }
}

pub fn store(line: AnchoredLine, x: i64) {
    if cfg!(target_pointer_width = "32") {
        line.set(0, Unit::from(x as i32));
        line.set(1, Unit::from((x >> 32) as i32));
    } else {
        line.set(0, Unit::from(x));
    }
}

pub fn hydrate(line: AnchoredLine) -> i64 {
    if cfg!(target_pointer_width = "32") {
        let low: u32 = line[0].into();
        let hi:  u32 = line[1].into();
        let res = ((hi as u64) << 32) | (low as u64);
        res as i64
    } else {
        line[0].into()
    }
}

impl Dispatch for Integral {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for Integral {
    fn type_name(&self) -> &'static str { "Integral" }
    fn type_sentinel(&self) -> *const u8 { (& INTEGRAL_SENTINEL) as *const u8 }
}

use std::cmp::Ordering;
impl Distinguish for Integral {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use hash::hash_64;
            let x = hydrate(guide.root) as u64;
            hash_64(x, 8)
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
        if o.type_sentinel() == (& INTEGRAL_SENTINEL) as *const u8 {
            let guide = Guide::hydrate(prism);
            let guide2 = Guide::hydrate(o.prism());
            let x = hydrate(guide.root);
            let y = hydrate(guide2.root);
            return Some(x.cmp(&y))
        }
        let ret = ((& INTEGRAL_SENTINEL) as *const u8).cmp(&o.type_sentinel());
        Some(ret)
    }
}

impl Aggregate for Integral { }
impl Sequential for Integral { }
impl Associative for Integral { }
impl Reversible for Integral {}
impl Sorted for Integral {}

impl Notation for Integral {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        let x = hydrate(guide.root);
        write!(f, "{}", x)
    }

    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integral[");
        self.edn(prism, f);
        write!(f, "]")
    }
}

impl Numeral for Integral {
    fn inc(&self, prism: AnchoredLine) -> Unit {
        let guide = Guide::hydrate(prism);
        let x = hydrate(guide.root);
        let s = guide.segment();
        if s.is_aliased() {
            if s.unalias() == 0 {
                Segment::free(s);
            }
            Integral::new(x + 1)
        } else {
            store(guide.root, x + 1);
            guide.clear_hash().store().segment().unit()
        }
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

// big int as string
use string;
pub static BIGINT_SENTINEL: u8 = 0;
pub struct BigInt {
    prism: Unit,
}
impl Dispatch for BigInt {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        prism[1].handle().retire();
        Segment::free(prism.segment())
    }
}
impl Identification for BigInt {
    fn type_name(&self) -> &'static str { "BigInt" }
    fn type_sentinel(&self) -> *const u8 { (& BIGINT_SENTINEL) as *const u8 }
}
impl Distinguish for BigInt {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        prism[1].handle().hash()
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.type_sentinel() != (& BIGINT_SENTINEL) as *const u8 {
            false
        } else {
            let s = prism.get(1).handle();
            let os = o.prism().get(1).handle();
            s.eq(os)
        }
    }
}
impl Aggregate for BigInt { }
impl Sequential for BigInt { }
impl Associative for BigInt { }
impl Reversible for BigInt {}
impl Sorted for BigInt {}
impl Notation for BigInt {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = string::guide::Guide::hydrate(prism[1].handle().prism());
        write!(f, "{}", guide.str())
    }
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BigInt[");
        self.edn(prism, f);
        write!(f, "]")
    }
}
impl Numeral for BigInt {}
pub fn big_int(negate: bool, m: &[u8]) -> Handle {
    use std::str::from_utf8;
    let temp = format!("{}{}N", if negate { "-" } else { "" }, from_utf8(m).unwrap());
    let t = string::Str::new_from_str(&temp).unit();
    let needed = 1 /*prism*/ + 1 /*string*/;
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, mechanism::prism::<BigInt>());
    prism.set(1, t);
    s.unit().handle()
}

#[cfg(test)]
mod tests {
    use super::*;

}
