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

use super::unit::Unit;
use super::anchor::{Anchor, AnchorLine};
use std::mem;

pub struct Segment {
    pub line: AnchorLine,
}

impl Segment {
    pub fn new(after_anchor_unit_count: usize) -> Segment {
        let cap = after_anchor_unit_count + 1; // for the anchor

        let unanchored: Segment = {
            let v: Vec<Unit> = Vec::with_capacity(cap);
            let ptr = v.as_ptr();
            mem::forget(v);
            Unit::from(ptr).into()
        };

        unanchored.line.set_anchor(Anchor::for_capacity(cap));
        let anchored = unanchored;
        anchored
    }

    pub fn capacity(&self) -> usize {
        self.line.get_anchor().capacity()
    }

    pub fn free(s: Segment) {
        unsafe {
            let cap = s.capacity();
            let v: Vec<Unit> =
                Vec::from_raw_parts(Unit::from(s).into(), 0, cap);
            mem::drop(v);
        }
    }

    pub fn copy_of_n(&self, n: usize) -> Segment {
        let mut cpy = Segment::new(n);
        for i in 1..(n + 1) {
            cpy[i] = src[i];
        }
        cpy
    }

    pub fn copy_of(&self) -> Segment {
        self.copy_of_n(self.capacity() - 1) // since we won't be copying the anchor
    }
}

use std::ops::{Index, IndexMut, Range, RangeTo};

impl Index<usize> for Segment {
    type Output = Unit;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            let anchor_line = self.line.line as *const Unit;
            & *anchor_line.offset(index as isize)
        }
    }
}

impl IndexMut<usize> for Segment {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            let anchor_line = self.line.line as *mut Unit;
            &mut *anchor_line.offset(index as isize)
        }
    }
}

impl Index<Range<usize>> for Segment {
    type Output = [Unit];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        use std::slice::from_raw_parts;
        let anchor_line = self.line.line as *mut Unit;
        unsafe {
            from_raw_parts(anchor_line.offset(index.start as isize),
                           (index.end - index.start))
        }
    }
}

impl Index<RangeTo<usize>> for Segment {
    type Output = [Unit];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.index(1..index.end)
    }
}
