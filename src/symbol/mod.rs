// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::Value;
use handle::Handle;

pub mod guide;
use self::guide::Guide;

pub static SYMBOL_SENTINEL: u8 = 0;

pub struct Symbol {
    prism: Unit,
}

impl Symbol {
    pub fn new(name: &[u8], solidus_position: u32) -> Unit {
        let byte_count = name.len() as u32;
        let content_count = units_for(byte_count);
        let needed = 1 /*prism*/ + Guide::units() + content_count;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Symbol>());
        let guide = Guide::new(prism, solidus_position, byte_count);
        guide.root.set(content_count as i32 - 1, Unit::zero());
        guide.byte_slice().copy_from_slice(name);
        guide.store().segment().unit()
    }

    pub fn new_prefix_name(prefix: &[u8], name: &[u8]) -> Unit {
        use std::str::from_utf8;
        let b = format!("{}/{}", from_utf8(prefix).unwrap(), from_utf8(name).unwrap());
        Symbol::new(b.as_bytes(), prefix.len() as u32)
    }
}

pub fn units_for(byte_count: u32) -> u32 {
    let (b, c) = if cfg!(target_pointer_width = "32") { (4, 2) } else { (8, 3) };
    (byte_count + b - 1) >> c
}

impl Dispatch for Symbol {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        let guide = Guide::hydrate(prism);
        guide.retire_meta();
        Segment::free(guide.segment())
    }
}

impl Identification for Symbol {
    fn type_name(&self) -> &'static str { "Symbol" }
    fn type_sentinel(&self) -> *const u8 { (& SYMBOL_SENTINEL) as *const u8 }
}

use std::cmp::Ordering;
impl Distinguish for Symbol {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix, mix_range, end};
            let iv: (u64, u64, u64, u64) = (PI[14], PI[15], PI[16], PI[17]);
            let unit_count = units_for(guide.count);
            let a = mix_range(guide.root.span(unit_count), iv);
            let (x, _y) = end(a.0, a.1, a.2, a.3);
            x as u32
        };
        guide.set_hash(h).store().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() && o.type_sentinel() == (& SYMBOL_SENTINEL) as *const u8 {
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
        if o.type_sentinel() != (& SYMBOL_SENTINEL) as *const u8 {
            let ret = ((& SYMBOL_SENTINEL) as *const u8).cmp(&o.type_sentinel());
            return Some(ret)
        }
        let g = Guide::hydrate(prism);
        let h = Guide::hydrate(o.prism());
        Some(g.str().cmp(&h.str()))
    }
}

impl Aggregate for Symbol {
    fn meta(&self, prism: AnchoredLine) -> *const Unit {
        let guide = Guide::hydrate(prism);
        if guide.has_meta() {
            guide.meta_line().line().star()
        } else {
            use handle::STATIC_NIL;
            (& STATIC_NIL) as *const Unit
        }
    }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit {
        unimplemented!()
    }
}

impl Sequential for Symbol { }
impl Associative for Symbol { }
impl Reversible for Symbol { }
impl Sorted for Symbol { }

impl Notation for Symbol {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "{}", guide.str())
    }

    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "Symbol[{}]", guide.str())
    }
}

impl Numeral for Symbol { }

#[cfg(test)]
mod tests {
    use super::*;
}

