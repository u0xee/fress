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
}
