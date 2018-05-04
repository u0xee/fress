use memory;
use memory::segment::Segment;
use memory::unit::Unit;

pub fn new_segment(count: u8) -> Segment {
    Segment::new(count)
}
