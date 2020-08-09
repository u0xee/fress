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
use integral;
use std::fmt::Debug;

pub struct Rational_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Rational_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_rational(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new(top: Handle, bot: Handle) -> Handle {
    assert!(integral::is_integral(top));
    assert!(integral::is_integral(bot));
    let s = Segment::new(3 /*prism numerator denominator*/);
    s.set(0, prism_unit());
    s.set(1, top.unit());
    s.set(2, bot.unit());
    s.unit().handle()
}
pub fn new_from_i64(top: i64, bot: i64) -> Handle {
    new(integral::new(top).handle(), integral::new(bot).handle())
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
    new_from_i64(x, y)
}

impl Dispatch for Rational_ { }
impl Identification for Rational_ {
    fn type_name(&self) -> &'static str { "Rational" }
}
impl Distinguish for Rational_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        use random::{PI, cycle_abc};
        let x = cycle_abc(75, PI[212].wrapping_add(prism[1].handle().hash() as u64));
        let z = cycle_abc(57, x.wrapping_add(prism[2].handle().hash() as u64));
        z as u32
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_rat) = find_prism(o) {
            prism[1].handle() == o_rat[1].handle() &&
                prism[2].handle() == o_rat[2].handle()
        } else {
            false
        }
    }
}

impl Aggregate for Rational_ { }
impl Sequential for Rational_ { }
impl Associative for Rational_ { }
impl Reversible for Rational_ {}
impl Sorted for Rational_ {}
impl Notation for Rational_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}/{}", prism[1].handle(), prism[2].handle()) // equivalent?
        prism[1].handle().fmt(f)?;
        write!(f, "/")?;
        prism[2].handle().fmt(f)
    }
}
impl Numeral for Rational_ { }
impl Callable for Rational_ { }

#[cfg(test)]
mod tests {
    use super::*;

}
