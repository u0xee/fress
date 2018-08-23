// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Store information in segments.
//!
//! A segment is a contiguous group of memory units.
//! We borrow segments from the memory pool, and later return them, after
//! the segment is no longer needed. Segments may be needed by several threads
//! at the same time.
//!
//! The first unit of a segment is its anchor. The anchor contains information about
//! its segment, such as the number of units it contains.
//! The anchor allows several threads to read information from the segment, sharing
//! the responsibility to return the segment to the memory pool when no longer needed.

use memory::unit::Unit;
use memory::anchor::Anchor;
use memory::line::Line;
use std::mem;
use std::ops::{Index, IndexMut};

pub struct Segment {
    pub line: Line,
}

impl Segment {
    pub fn new(after_anchor_unit_count: u32) -> Segment {
        Segment::with_capacity(1 + after_anchor_unit_count)
    }

    pub fn with_capacity(capacity: u32) -> Segment {
        let mut unanchored: Segment = {
            let v: Vec<Unit> = Vec::with_capacity(capacity as usize);
            let ptr = v.as_ptr();
            mem::forget(v);
            Unit::from(ptr).into()
        };

        unanchored.line[0] = Anchor::for_capacity(capacity).into();
        let anchored = unanchored;
        anchored
    }

    pub fn free(s: Segment) {
        unsafe {
            let cap = s.capacity();
            let v: Vec<Unit> =
                Vec::from_raw_parts(Unit::from(s).into(), 0,
                                    cap as usize);
            mem::drop(v);
        }
    }

    pub fn capacity(&self) -> u32 {
        Anchor::from(self.line[0]).capacity()
    }

    pub fn is_aliased(&self) -> bool {
        Anchor::from(self.line[0]).is_aliased()
    }

    pub fn alias(&mut self) {
        // TODO CAS
        let a: usize = self.line[0].into();
        self.line[0] = (a + 1).into();
    }

    pub fn unalias(&mut self) -> u32 {
        // TODO CAS
        let a: usize = self.line[0].into();
        self.line[0] = (a - 1).into();
        Anchor::from(self.line[0]).aliases()
    }
}

impl From<Unit> for Segment {
    fn from(u: Unit) -> Self {
        Segment { line: u.into() }
    }
}

impl From<Line> for Segment {
    fn from(line: Line) -> Self {
        Segment { line: line }
    }
}

impl Index<usize> for Segment {
    type Output = Unit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.line[index]
    }
}

impl IndexMut<usize> for Segment {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.line[index]
    }
}

impl Index<u32> for Segment {
    type Output = Unit;

    fn index(&self, index: u32) -> &Self::Output {
        &self.line[index as usize]
    }
}

impl IndexMut<u32> for Segment {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.line[index as usize]
    }
}

impl Index<i32> for Segment {
    type Output = Unit;

    fn index(&self, index: i32) -> &Self::Output {
        &self.line[index as usize]
    }
}

impl IndexMut<i32> for Segment {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.line[index as usize]
    }
}
