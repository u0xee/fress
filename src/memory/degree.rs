// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Track the in-degree of segments

use memory::segment::Segment;
use memory::unit::Unit;

pub fn inc(u: Unit) {
    if u.is_even() {
        // do the work
    }
}

pub fn dec(u: Unit) -> Option<Segment> {
    if u.is_even() {
        // do the work
    }
}

impl Segment {
    pub fn is_mine(&self) -> bool {
        unimplemented!()
    }
}

// compile-time flag to switch between strategies
