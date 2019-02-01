// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::ops::Range;
use std::fmt;
use memory::*;
use value::*;

#[derive(Copy, Clone, Debug)]
pub struct AnchoredRange {
    pub seg: Segment,
    pub start: u32,
    pub end: u32,
}

impl AnchoredRange {
    pub fn new(seg: Segment, range: Range<u32>) -> AnchoredRange {
        AnchoredRange { seg, start: range.start, end: range.end }
    }

    pub fn segment(&self) -> Segment {
        self.seg
    }

    pub fn span(&self) -> u32 {
        self.end - self.start
    }

    pub fn to(&self, target: Segment) {
        self.to_offset(target, self.start)
    }

    pub fn to_offset(&self, target: Segment, offset: u32) {
        // TODO make efficient aggregate operation, will bounds and mut check in loop
        let mut t = target;
        let length = self.end - self.start;
        for i in 0..length {
            t[offset + i] = self.seg[self.start + i];
        }
    }

    pub fn shift_up(&self, shift: u32) {
        let mut t = self.seg;
        for i in (self.start..self.end).rev() {
            t[i + shift] = t[i];
        }
    }

    pub fn shift_down(&self, shift: u32) {
        let mut t = self.seg;
        for i in self.start..self.end {
            t[i - shift] = t[i];
        }
    }

    pub fn each_unit<F: FnMut(Unit)>(&self, mut f: F) {
        for i in self.start..self.end {
            f(self.seg[i])
        }
    }

    pub fn alias(&self) {
        self.each_unit(|u| u.segment().alias());
    }

    pub fn unalias(&self) {
        self.each_unit(|u| if u.segment().unalias() == 0 {
            panic!("Unalias of segment w/o a policy on how to free!")
        });
    }
}

impl AnchoredRange {
    pub fn split(&self) {
        self.each_unit(|u| u.handle().split());
    }

    pub fn retire(&self) {
        self.each_unit(|u| u.handle().retire());
    }

    pub fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.span() == 0 {
            write!(f, "")
        } else {
            let mut short = *self;
            short.end -= 1;
            short.each_unit(|u| { write!(f, "{:?} ", u.handle()); });
            write!(f, "{:?}", self.seg[self.end - 1].handle())
        }
    }
}
