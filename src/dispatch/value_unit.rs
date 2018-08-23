// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use memory::segment::Segment;
use dispatch::*;

pub struct ValueUnit {
    pub unit: Unit,
}

impl ValueUnit {
    pub fn split(&self) {
        if self.unit.is_even() {
            Segment::from(self.unit).alias()
        }
    }

    pub fn retire(&self) {
        if self.unit.is_even() {
            let mut s = Segment::from(self.unit);
            if s.unalias() == 0 {
                s.tear_down()
            }
        }
    }
}

impl From<Unit> for ValueUnit {
    fn from(u: Unit) -> Self {
        ValueUnit { unit: u }
    }
}

impl Into<Unit> for ValueUnit {
    fn into(self) -> Unit {
        self.unit
    }
}

