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
use handle::Handle;
use symbol;
use symbol::guide::Guide;
use std::cmp::Ordering;

pub struct Keyword_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Keyword_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_keyword(h: Handle) -> bool { find_prism(h).is_some() }


// TODO lightweight "parse" from string
//  for creation (often) from cache
pub fn new_full_name(full_name: &str) -> Unit {
    // assert start with : colon
    // assert no slash, or exactly one slash (not at beginning or end)
    unimplemented!()
}
pub fn new(name: &[u8], solidus_position: u32) -> Unit {
    //log!("new keyword {}", from_utf8(name).unwrap());
    // TODO intern based on flag
    // global, thread local, hash set
    new_(name, solidus_position)
}
pub fn new_prefix_name(prefix: &[u8], name: &[u8]) -> Unit {
    let b = format!(":{}/{}", from_utf8(prefix).unwrap(), from_utf8(name).unwrap());
    new(b.as_bytes(), prefix.len() as u32 + 1)
}
pub fn new_from_name(name: &[u8]) -> Unit {
    let b = format!(":{}", from_utf8(name).unwrap());
    new(b.as_bytes(), 0)
}

pub fn new_(name: &[u8], solidus_position: u32) -> Unit {
    let byte_count = name.len() as u32;
    let content_count = symbol::units_for(byte_count);
    let needed = 1 /*prism*/ + Guide::units() + content_count;
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, prism_unit());
    let guide = Guide::new(prism, solidus_position, byte_count);
    guide.root.set(content_count as i32 - 1, Unit::zero());
    guide.byte_slice().copy_from_slice(name);
    guide.store().segment().unit()
}

// TODO global ID counter (for unique ids on read, gensym
// Global to all threads?
// flag to intern or not, in constructor
// map of interned keys
use std::cell::Cell;
thread_local! {
    pub static KEYWORD_CACHE: Cell<u32> = Cell::new(0);
}
pub fn x() {
    KEYWORD_CACHE.with(|c| c.set(0));
}

pub fn has_namespace(prism: AnchoredLine) -> bool {
    assert!(is_prism(prism));
    let guide = Guide::hydrate(prism);
    guide.solidus != 0
}

impl Dispatch for Keyword_ { /*default tear_down, alias_components*/ }
impl Identification for Keyword_ {
    fn type_name(&self) -> &'static str { "Keyword" }
}
impl Distinguish for Keyword_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix_range, end};
            let iv: (u64, u64, u64, u64) = (PI[14], PI[15], PI[16], PI[17]);
            let unit_count = symbol::units_for(guide.count);
            // <= 4x64 bits wide, do on stack
            let a = mix_range(guide.root.span(unit_count), iv);
            let (x, _y) = end(a.0, a.1, a.2, a.3);
            x as u32
        };
        //log!("Keyword hash: {} {:#08X}", prism.segment().unit().handle(), h);
        //prism.segment().print_bits();
        guide.set_hash(h).store_hash(); // TODO
        h
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_key) = find_prism(o) {
            //log!("Keyword eq: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_key);
            return g.byte_slice() == h.byte_slice()
        } else {
            false
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_key) = find_prism(o) {
            //log!("Keyword cmp: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_key);
            Some(g.str().cmp(&h.str()))
        } else {
            if o.is_ref() {
                let o_prism_unit = o.logical_value()[0];
                Some(prism_unit().cmp(&o_prism_unit))
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}
impl Aggregate for Keyword_ { }
impl Associative for Keyword_ { }
impl Sequential for Keyword_ { }
impl Reversible for Keyword_ { }
impl Sorted for Keyword_ { }
impl Notation for Keyword_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "{}", guide.str())
    }
}
impl Numeral for Keyword_ { }
impl Callable for Keyword_ {
    fn invoke1(&self, prism: AnchoredLine, a: Unit) -> Unit {
        // lookup using keyword
        // hash and Handle.get
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

