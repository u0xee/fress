// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use std::fmt;
use memory::*;
use dispatch::*;
use symbol::guide::Guide;
use symbol::units_for;

pub static KEYWORD_SENTINEL: u8 = 0;

pub struct Keyword { }

impl Keyword {
    pub fn new(name: &[u8], solidus_position: u32) -> Unit {
        log!("new keyword {}", from_utf8(name).unwrap());
        // TODO intern based on compile flag
        Keyword::new_(name, solidus_position)
    }

    pub fn new_prefix_name(prefix: &[u8], name: &[u8]) -> Unit {
        let b = format!(":{}/{}", from_utf8(prefix).unwrap(), from_utf8(name).unwrap());
        Keyword::new(b.as_bytes(), prefix.len() as u32 + 1)
    }

    pub fn new_from_name(name: &[u8]) -> Unit {
        let b = format!(":{}", from_utf8(name).unwrap());
        Keyword::new(b.as_bytes(), 0)
    }

    pub fn new_(name: &[u8], solidus_position: u32) -> Unit {
        let byte_count = name.len() as u32;
        let content_count = units_for(byte_count);
        let needed = 1 /*prism*/ + Guide::units() + content_count;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Keyword>());
        let guide = Guide::new(prism, solidus_position, byte_count);
        guide.root.set(content_count as i32 - 1, Unit::zero());
        guide.byte_slice().copy_from_slice(name);
        guide.store().segment().unit()
    }
}

impl Dispatch for Keyword {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        log!("keyword tear down {}", prism.segment().unit().handle());
        let guide = Guide::hydrate(prism);
        Segment::free(guide.segment())
    }
}

impl Identification for Keyword {
    fn type_name(&self) -> &'static str { "Keyword" }
    fn type_sentinel(&self) -> *const u8 { (& KEYWORD_SENTINEL) as *const u8 }
}

use std::cmp::Ordering;
impl Distinguish for Keyword {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix_range, end};
            let iv: (u64, u64, u64, u64) = (PI[14], PI[15], PI[16], PI[17]);
            let unit_count = units_for(guide.count);
            let a = mix_range(guide.root.span(unit_count), iv);
            let (x, _y) = end(a.0, a.1, a.2, a.3);
            x as u32
        };
        log!("hash keyword {} {:#08X}", prism.segment().unit().handle(), h);
        guide.set_hash(h).store_hash().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_ref() && o.type_sentinel() == (& KEYWORD_SENTINEL) as *const u8 {
            log!("keyword eq");
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
        if o.type_sentinel() != (& KEYWORD_SENTINEL) as *const u8 {
            let ret = ((& KEYWORD_SENTINEL) as *const u8).cmp(&o.type_sentinel());
            return Some(ret)
        }
        let g = Guide::hydrate(prism);
        let h = Guide::hydrate(o.prism());
        Some(g.str().cmp(&h.str()))
    }
}

impl Aggregate for Keyword { }
impl Associative for Keyword { }
impl Sequential for Keyword { }
impl Reversible for Keyword { }
impl Sorted for Keyword { }

impl Notation for Keyword {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "{}", guide.str())
    }
}

impl Numeral for Keyword { }
impl Callable for Keyword { }

#[cfg(test)]
mod tests {
    use super::*;
}

