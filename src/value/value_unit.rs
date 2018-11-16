// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
use dispatch::*;

#[derive(Copy, Clone)]
pub struct ValueUnit {
    pub unit: Unit,
}

#[derive(Copy, Clone)]
pub struct ValueRef {
    pub seg: Segment,
}

#[derive(Copy, Clone)]
pub struct ValueImm {
    pub unit: Unit,
}

impl ValueUnit {
    pub fn as_ref(&self) -> Option<ValueRef> {
        if self.unit.is_even() {
            Some(ValueRef { seg: self.unit.segment() })
        } else {
            None
        }
    }

    pub fn split(&self) {
        if let Some(r) = self.as_ref() {
            r.seg.alias()
        }
    }

    pub fn retire(&self) {
        if let Some(r) = self.as_ref() {
            if r.seg.unalias() == 0 {
                r.tear_down()
            }
        }
    }

    pub fn hash(&self) -> u32 {
        if let Some(r) = self.as_ref() {
            r.hash()
        } else {
            // TODO hash immediate value
            self.unit.into()
        }
    }

    pub fn eq(&self, other: Unit) -> bool {
        self.unit == other
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

