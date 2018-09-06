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
}

impl Dispatch for BlankShim {
    fn tear_down(&self) {
        unimplemented!()
    }
}

impl fmt::Display for BlankShim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
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
impl Aggregate for BlankShim {}
impl Sequential for BlankShim {}
impl Associative for BlankShim {}
impl Reversible for BlankShim {}
impl Sorted for BlankShim {}
impl Named for BlankShim {}
