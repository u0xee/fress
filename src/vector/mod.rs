// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Indexed array mapped trie, supporting vectors and lists.

use std::fmt;
use memory::*;
use dispatch::*;
use value::*;
use handle::Handle;

pub mod guide;
use self::guide::Guide;
pub mod conj;
use self::conj::unaliased_root;
pub mod pop;
pub mod nth;
pub mod meta;
pub mod assoc;
pub mod tear_down;
pub mod util;
use self::util::*;
#[cfg(test)]
use fuzz;

/// Defines branching factor.
///
/// Can be 4, 5 or 6, making for sixteen, thirty-two or sixty-four way branching.
pub const BITS: u32 = 4; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

pub static VECTOR_SENTINEL: u8 = 0;

/// Vector dispatch.
pub struct Vector {
    prism: Unit,
}

impl Vector {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(6);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Vector>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g.is_compact_bit = 0x1;
            g
        };
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Vector::new().handle().value()
    }
}

impl Dispatch for Vector {
    fn tear_down(&self, prism: AnchoredLine) {
        tear_down::tear_down(prism);
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unaliased_root(Guide::hydrate(prism)).segment().unit()
    }
}

impl Identification for Vector {
    fn type_name(&self) -> String {
        "Vector".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }
}

impl Distinguish for Vector {}

impl Aggregate for Vector {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        Vector::new()
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        conj::conj(prism, x)
    }
    fn meta(&self, prism: AnchoredLine) -> Unit {
        meta::meta(prism)
    }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit {
        meta::with_meta(prism, m)
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        pop::pop(prism)
    }
}

impl Sequential for Vector {
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        nth::nth(prism, idx).line().star()
    }
}

fn key_into_idx(k: Unit) -> u32 {
    // TODO need general conversion to int
    let i: u32 = k.into();
    i >> 4
}

impl Associative for Vector {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let idx: u32 = key_into_idx(k);
        assoc::assoc(prism, idx, v)
    }
}

impl Reversible for Vector {}
impl Sorted for Vector {}
impl Notation for Vector {
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide= Guide::hydrate(prism);
        let hash = if !guide.has_hash() { "".to_string() } else {
            format!(" #x{:X}", guide.hash)
        };
        let meta = if !guide.has_meta() { "".to_string() } else {
            format!(" ^{:?}", guide.meta_line()[0].handle())
        };
        write!(f, "{aliases}->[Vector{hash}{meta} {count}ct ",
               aliases = guide.segment().anchor().aliases(),
               hash = hash, meta = meta, count = guide.count);
        if guide.count <= TAIL_CAP {
            if guide.is_compact_bit == 0 { write!(f, "_ "); }
            guide.root.span(guide.count).debug(f);
            let used = guide.root.index + guide.count;
            let empty = guide.segment().capacity() - used;
            if empty != 0 { write!(f, " _{}", empty); }
            write!(f, "]")
        } else {
            let tail = guide.root[-1].segment();
            write!(f, "tail {}->[", tail.anchor().aliases());
            tail.at(0..tail_count(guide.count)).debug(f);
            //let rc = root_content_count(tailoff(guide.count));
            //let last_index = tailoff(guide.count) - 1;
            // root elems are value units
            // root elems are nodes
            //
            //guide.root.span(rc).debug(f);
            //write!(f, "]")
            write!(f, "]]\n")
        }
    }

    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
