// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use Value;

pub mod guide;
use self::guide::Guide;
mod conj;
use self::conj::unalias_root;
mod pop;
mod nth;
mod meta;
mod assoc;
mod tear_down;
mod util;
use self::util::*;

pub const BITS: u32 = 5; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

pub static VECTOR_SENTINEL: u8 = 0;

pub struct Vector {
    prism: Unit,
}

impl Vector {
    pub fn new() -> Unit {
        // TODO randomize root_gap and anchor_gap under test build
        let mut s = Segment::new(6);
        s[1] = prism::<Vector>();
        s[2] = Guide::new().into();
        Unit::from(s)
    }

    fn line(&self) -> Line {
        Unit::from(&self.prism as *const Unit).into()
    }
}

impl Dispatch for Vector {
    fn tear_down(&self) {
        tear_down::tear_down(self.line())
    }
    fn anchor_gap_change(&self, delta: i32) {
        let mut prism = self.line();
        let guide: Guide = prism[1].into();
        prism[1] = guide.with_anchor_gap_change(delta).into();
    }
    fn unaliased(&self) -> Unit {
        let prism = self.line();
        let guide: Guide = prism[1].into();
        let anchor_gap = guide.prism_to_anchor_gap();
        let root_gap = guide.guide_to_root_gap();
        let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
        if !segment.is_aliased() {
            Unit::from(segment)
        } else {
            let tailoff = (guide.count() - 1) & !MASK;
            let t = unalias_root(segment, anchor_gap,
                                 root_gap,root_content_count(tailoff), guide);
            Unit::from(t)
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
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
    fn count(&self) -> u32 {
        count(self.line())
    }
    fn conj(&self, x: Unit) -> Unit {
        conj::conj(self.line(), x)
    }
    fn meta(&self) -> Unit {
        meta::meta(self.line())
    }
    fn with_meta(&self, m: Unit) -> Unit {
        meta::with_meta(self.line(), m)
    }
    fn pop(&self) -> (Unit, Unit) {
        pop::pop(self.line())
    }
}

impl Sequential for Vector {
    fn nth(&self, idx: u32) -> Unit {
        nth::nth(self.line(), idx)
    }
}

fn key_into_idx(k: Unit) -> u32 {
    // TODO need general conversion to int
    let i: u32 = k.into();
    i >> 4
}

impl Associative for Vector {
    fn assoc(&self, k: Unit, v: Unit) -> (Unit, Unit) {
        let idx: u32 = key_into_idx(k);
        assoc::assoc(self.line(), idx, v)
    }
}

impl Reversible for Vector {}
impl Sorted for Vector {}
impl Named for Vector {}


pub fn count(prism: Line) -> u32 {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    count
}

pub fn has_tail_space(guide: Guide) -> bool {
    let x: u64 = guide.post.into();
    ((x >> 52) & 1) == 1
}

pub fn with_tail_space(guide: Guide) -> Guide {
    let x: u64 = guide.post.into();
    Unit::from(x | (1 << 52)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

}
