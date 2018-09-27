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

pub struct BlankShim {
    prism: Unit,
}

pub static BLANKSHIM_SENTINEL: u8 = 0;

impl BlankShim {
    pub fn on_top_of(v: Segment, gap: u32) -> Segment {
        let mut s: Segment = v.unaliased().into();
        s.anchor_gap_change((2 + gap) as i32);
        let s_cap = s.capacity();
        let mut t = Segment::with_capacity(s_cap + 2 /*new prism and guide*/ + gap);
        t[1] = prism::<BlankShim>();
        t[2] = gap.into();
        for i in 1..s_cap {
            t[2 + gap + i] = s[i]; // start at index 3 (no gap)
        }
        Segment::free(s);
        t
    }

    fn line(&self) -> Line {
        Unit::from(&self.prism as *const Unit).into()
    }

    fn gap(&self) -> u32 {
        self.line()[1].into()
    }

    fn next_prism(&self) -> Line {
        self.line().offset((2 + self.gap()) as isize)
    }
}

impl Dispatch for BlankShim {
    fn tear_down(&self) {
        self.next_prism().tear_down()
    }

    fn unaliased(&self) -> Unit {
        self.next_prism().unaliased()
    }
}

impl fmt::Display for BlankShim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.next_prism().fmt(f)
    }
}

impl Identification for BlankShim {
    fn type_name(&self) -> String {
        "BlankShim".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& BLANKSHIM_SENTINEL) as *const u8
    }
}

impl Distinguish for BlankShim {}
impl Aggregate for BlankShim {
    fn count(&self) -> u32 {
        self.next_prism().count()
    }
    fn conj(&self, x: Unit) -> Unit {
        self.next_prism().conj(x)
    }
    fn meta(&self) -> Unit {
        self.next_prism().meta()
    }
    fn with_meta(&self, m: Unit) -> Unit {
        self.next_prism().with_meta(m)
    }
    fn pop(&self) -> (Unit, Unit) {
        self.next_prism().pop()
    }
}
impl Sequential for BlankShim {
    fn nth(&self, idx: u32) -> Unit {
        self.next_prism().nth(idx)
    }
}
impl Associative for BlankShim {
    fn assoc(&self, k: Unit, v: Unit) -> (Unit, Unit) {
        self.next_prism().assoc(k, v)
    }
}
impl Reversible for BlankShim {}
impl Sorted for BlankShim {}
impl Named for BlankShim {}

