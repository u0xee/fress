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

pub static RATIONAL_SENTINEL: u8 = 0;

pub struct Rational { }

impl Rational {
    pub fn new(top: i64, bot: i64) -> Unit {
        // A | P T B
        let s = Segment::new(if cfg!(target_pointer_width = "32") { 5 } else { 3 });
        s.set(0, mechanism::prism::<Rational>());
        store(s.line_at(0), top);
        let next = if cfg!(target_pointer_width = "32") { 2 } else { 1 };
        store(s.line_at(next), bot);
        s.unit()
    }

    pub fn is_instance(h: Handle) -> bool {
        h.is_ref() && h.type_sentinel() == (& RATIONAL_SENTINEL) as *const u8
    }

    pub fn parse(negate: bool, top: &[u8], bot: &[u8]) -> Handle {
        let mut x = 0i64;
        for b in top.iter() {
            if *b == b'_' {
                continue
            }
            x = x * 10 + (*b - b'0') as i64;
        }
        let mut y = 0i64;
        for b in bot.iter() {
            if *b == b'_' {
                continue
            }
            y = y * 10 + (*b - b'0') as i64;
        }
        if negate { x = -x; }
        Rational::new(x, y).handle()
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
    fn type_name(&self) -> &'static str { "Rational" }
    fn type_sentinel(&self) -> *const u8 { (& RATIONAL_SENTINEL) as *const u8 }
}

//use std::cmp::Ordering;
impl Distinguish for Rational {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let x = hydrate(prism) as u64;
        let next = if cfg!(target_pointer_width = "32") { 2 } else { 1 };
        let y = hydrate(prism.offset(next)) as u64;
        use hash::hash_128;
        hash_128(x, y, 16)
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.type_sentinel() == (& RATIONAL_SENTINEL) as *const u8 {
            let next = if cfg!(target_pointer_width = "32") { 2 } else { 1 };
            let x =  hydrate(prism);
            let y =  hydrate(prism.offset(next));
            let oprism = o.prism();
            let ox = hydrate(oprism);
            let oy = hydrate(oprism.offset(next));
            return x == ox && y == oy
        }
        false
    }
}

impl Aggregate for Rational { }

impl Sequential for Rational { }

impl Associative for Rational { }

impl Reversible for Rational {}
impl Sorted for Rational {}

impl Notation for Rational {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let next = if cfg!(target_pointer_width = "32") { 2 } else { 1 };
        let x =  hydrate(prism);
        let y =  hydrate(prism.offset(next));
        write!(f, "{}/{}", x, y)
    }
}

impl Numeral for Rational { }
impl Callable for Rational { }

#[cfg(test)]
mod tests {
    use super::*;

}
