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
