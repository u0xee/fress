// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std;
use std::fmt;
use std::io;
use memory::*;
use dispatch::*;
use transduce::{Process};
use value::Value;

pub mod guide;
use self::guide::Guide;

pub static STR_SENTINEL: u8 = 0;

pub struct Str {
    prism: Unit,
}

impl Str {
    pub fn new() -> Unit {
        let s = Segment::new(8);
        s.set(0, mechanism::prism::<Str>());
        s.set(1, Unit::from(0));
        s.unit()
    }

    pub fn new_from_str(source: &str) -> Unit {
        let bytes = source.len() as u32;
        use std::cmp;
        let units = cmp::max(1, units_for(bytes));
        let needed = 1 /*prism*/ + Guide::units() + units;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Str>());
        let guide  = Guide::hydrate_top_bot(prism, 0, bytes);
        for i in 0..units {
            guide.root.set(i as i32, Unit::zero());
        }
        guide.byte_slice().copy_from_slice(source.as_bytes());
        guide.store().segment().unit()
    }

    pub fn new_value_from_str(source: &str) -> Value {
        Str::new_from_str(source).handle().value()
    }
}

pub fn units_for(byte_count: u32) -> u32 {
    let b = Unit::bytes();
    (byte_count + b - 1) / b
}

impl Dispatch for Str {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
}

impl Identification for Str {
    fn type_name(&self) -> &'static str {
        "String"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& STR_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Str {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        if Unit::bytes() == 4 {
            unimplemented!("32bit str hash")
        }
        let guide = Guide::hydrate(prism);
        let unit_count = units_for(guide.count);
        use random::PI;
        let mut a: [u64; 4] = [PI[0], PI[1], PI[2], PI[3]];
        let mut remain = unit_count;
        while remain > 4 {
            let idx = (unit_count - remain) as i32;
            a[0] ^= guide.root[idx].u64();
            a[1] ^= guide.root[idx + 1].u64();
            a[2] ^= guide.root[idx + 2].u64();
            a[3] ^= guide.root[idx + 3].u64();
            {
                use hash::mix;
                let m = mix(a[0], a[1], a[2], a[3]);
                a[0] = m.0; a[1] = m.1; a[2] = m.2; a[3] = m.3;
            }
            remain -= 4;
        }
        let idx = (unit_count - remain) as i32;
        if remain > 0 { a[0] ^= guide.root[idx].u64(); }
        if remain > 1 { a[1] ^= guide.root[idx + 1].u64(); }
        if remain > 2 { a[2] ^= guide.root[idx + 2].u64(); }
        if remain > 3 { a[3] ^= guide.root[idx + 3].u64(); }
        let h = {
            use hash::{mix, end};
            let m = mix(a[0], a[1], a[2], a[3]);
            let (x, y) = end(m.0, m.1, m.2, m.3);
            x as u32
        };
        guide.set_hash(h).store().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() && o.type_sentinel() == (& STR_SENTINEL) as *const u8 {
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o.prism());
            return g.byte_slice() == h.byte_slice()
        }
        false
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if !o.is_ref() {
            return Some(Ordering::Greater)
        }
        if o.type_sentinel() != (& STR_SENTINEL) as *const u8 {
            let ret = ((& STR_SENTINEL) as *const u8).cmp(&o.type_sentinel());
            return Some(ret)
        }
        let g = Guide::hydrate(prism);
        let h = Guide::hydrate(o.prism());
        Some(g.str().cmp(&h.str()))
    }
}

impl Aggregate for Str {
    fn count(&self, prism: AnchoredLine) -> u32 {
        unimplemented!()
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        unimplemented!()
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        unimplemented!()
    }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<Process>]) -> Value {
        unimplemented!()
    }
}
impl Sequential for Str { }
impl Associative for Str { }
impl Reversible for Str {}
impl Sorted for Str {}
impl Notation for Str {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "\"{}\"", guide.str())
    }

    fn fressian(&self, prism:AnchoredLine, w: &mut io::Write) -> io::Result<usize> {
        unimplemented!()
    }
}
impl Numeral for Str {}

#[cfg(test)]
mod tests {
    use super::*;

}
