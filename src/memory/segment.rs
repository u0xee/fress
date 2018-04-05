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

pub struct Segment {
    line: AnchorLine,
}
