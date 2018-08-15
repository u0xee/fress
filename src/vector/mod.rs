// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::unit::Unit;
use memory::segment::Segment;
use memory::line::Line;
use bit::{bottom_32, top_32, top_16, MASK_32, MASK_16, top_byte, with_top_byte, second_top_byte, clear_top, clear_bottom, splice, with_second_top_byte};
use dispatch;
use dispatch::Dispatch;
use Value;

pub mod tree;
pub mod memory;
pub mod guide;

use self::guide::Guide;

pub const BITS: u32 = 5; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

pub static VECTOR_SENTINEL: u8 = 0;

pub struct Vector {
    distributor: Unit,
}



impl Vector {
    pub fn new() -> Value {
        let s = Segment::new(6);
        s[1] = distributor::<Vector>();
        s[2] = Guide::new().into();
        Value { handle: Unit::from(s.pointer_to_unit(1)) }
    }
}

fn tree_count(count: u32) -> u32 {
    (count - 1) & !MASK
}

fn significant_bits(x: u32) -> u8 {
    /*bits in a u32*/ 32 - x.leading_zeros() as u8
}

fn digit_count(x: u32) -> u8 {
    ((significant_bits(x) as u32 + BITS - 1) as u32 / BITS) as u8
}

fn digit(x: u32, idx: u8) -> u8 {
    (x >> (idx as u32 * BITS)) as u8
}

fn digit_iter(x: u32, digits: u8) {
    // Digit iterator struct
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

use dispatch::{Identification, Distinguish, AggregateAbstractions, StreamlinedMethods};

impl Identification for Vector {
    fn type_name(&self) -> String {
        "Vector".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Vector {
    fn hash(&self) -> u32 {
        unimplemented!()
    }

    fn eq(&self, other: &Dispatch) -> bool {
        unimplemented!()
    }

    fn cmp(&self, other: &Dispatch) -> Ordering {
        unimplemented!()
    }
}

impl AggregateAbstractions for Vector {
}


fn conj(prism: Line, x: Unit) -> Unit {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    if count <= TAIL_CAP {
        conj_untailed(prism, x, count)
    } else {
        conj_tailed(prism, x, count)
    }
}

fn conj_untailed(prism: Line, x: Unit, count: u32) -> Unit {
    if count == TAIL_CAP {
        conj_untailed_complete(prism, x, count)
    } else {
        conj_untailed_incomplete(prism, x, count)
    }
}

fn conj_untailed_complete(prism: Line, x: Unit, count: u32) -> Unit {

}

fn conj_untailed_incomplete(prism: Line, x: Unit, count: u32) -> Unit {
    let guide: Guide = prism[1].into();
    let anchor_gap = guide.prism_to_anchor_gap();
    let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    if segment.is_aliased() {
        conj_untailed_incomplete_aliased(prism, x, count, anchor_gap, segment)
    } else {
        conj_untailed_incomplete_unaliased(prism, x, count, anchor_gap, segment)
    }
}

fn conj_untailed_incomplete_aliased(prism: Line, x: Unit, count: u32,
                                    anchor_gap: u32, segment: Segment) -> Unit {
    let guide: Guide = prism[1].into();
    let root_gap = guide.guide_to_root_gap();
    let used_units = anchor_gap + root_gap + count + 3 /*anchor, prism, guide*/;

    let new_count = (count + 1).next_power_of_two();
    let new_cap = used_units + (new_count - count);
    let (shift, guide) = if new_count == TAIL_CAP
        { (1, guide.inc_guide_to_root_gap()) } else { (0, guide) };
    let s = Segment::with_capacity(new_cap + shift);

    for i in 1..(used_units - count) {
        s[i] = segment[i]
    }
    for i in (used_units - count)..used_units {
        s[i + shift] = segment[i]
    }
    // increment each item's alias count
    for i in (used_units - count)..used_units {
        s[i + shift]
    }
    // increment Meta's alias count


    s[used_units + shift] = x;
    s[2 + anchor_gap] = guide.inc().into();
    Line::from(s).offset((1 + anchor_gap) as isize).into()
}

fn conj_untailed_incomplete_unaliased(prism: Line, x: Unit, count: u32,
                                    anchor_gap: u32, segment: Segment) -> Unit {
    let guide: Guide = prism[1].into();
    let root_gap = guide.guide_to_root_gap();
    let used_units = anchor_gap + root_gap + count + 3 /*anchor, prism, guide*/;
    let cap = segment.capacity();

    if used_units == cap {
        let new_count = (count + 1).next_power_of_two();
        let new_cap = used_units + (new_count - count);
        let (shift, guide) = if new_count == TAIL_CAP
            { (1, guide.inc_guide_to_root_gap()) } else { (0, guide) };
        let s = Segment::with_capacity(new_cap + shift);

        for i in 1..(used_units - count) {
            s[i] = segment[i]
        }
        for i in (used_units - count)..used_units {
            s[i + shift] = segment[i]
        }

        s[used_units + shift] = x;
        s[2 + anchor_gap] = guide.inc().into();
        Line::from(s).offset((1 + anchor_gap) as isize).into()
    } else {
        segment[used_units] = x;
        segment[2 + anchor_gap] = guide.inc().into();
        Line::from(segment).offset((1 + anchor_gap) as isize).into()
    }
}

fn conj_tailed(prism: Line, x: Unit, count: u32) -> Unit {

}

impl Vector {
    fn line(&self) -> Line {
        Unit::from(&self.distributor as *const Unit).into()
    }
}

impl StreamlinedMethods for Vector {
    fn conj(&mut self, x: Value) -> Value {
        let res = conj(self.line(), x.handle);
        // mem::forget(x) ???
        Value { handle: res }
    }
}

impl Dispatch for Vector {
}

impl Drop for Vector {
    fn drop(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
