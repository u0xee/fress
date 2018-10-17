// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Store information in segments.
//!
//! A segment is a contiguous group of memory units.
//! We borrow a segment from the memory pool, and later return it, when
//! the segment is no longer needed. A segment is no longer needed when
//! no construct exists which references it. No references from the stack,
//! nor from other segments.
//!
//! The first unit of a segment is its anchor. The anchor contains information about
//! its segment, such as the number of units it contains.
//! The anchor allows one or more threads to read information from the segment, sharing
//! the responsibility to return the segment to the memory pool, when no longer needed.

use memory::unit::Unit;
use memory::anchor::Anchor;
use memory::line::Line;
use std::mem;
use std::fmt;
use std::ops::{Index, IndexMut};
use fuzz;

#[derive(Copy, Clone)]
pub struct Segment {
    pub line: Line,
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        string.push_str("[");
        string.push_str(&format!("{:?}", self[1]));
        for i in 2..self.capacity() {
            string.push_str(", ");
            string.push_str(&format!("{:?}", self[i]));
        }
        string.push_str("]");
        write!(f, "Segment {:p}, {:?}\n    {}",
               self.line.line, Anchor::from(self.line[0]),
               string)
    }
}

impl Segment {
    pub fn new(after_anchor_unit_count: u32) -> Segment {
        #[cfg(any(test, feature = "segment_clear"))]
            println!("clearing segments...");
        if cfg!(any(test, feature = "segment_clear")) {
            println!("with clearing!!!!")
        } else {
            println!("without clearing")
        }
        Segment::with_capacity(1 + after_anchor_unit_count)
    }

    pub fn with_capacity_internal(capacity: u32) -> Segment {
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

    #[cfg(not(test))]
    pub fn with_capacity(capacity: u32) -> Segment {
        Segment::with_capacity_internal(capacity)
    }

    #[cfg(test)]
    pub fn with_capacity(capacity: u32) -> Segment {
        let (seed, log_tail) = fuzz::next_random();
        let p = fuzz::uniform_f64(seed, fuzz::cycle(seed));
        let extra_cap = if p < 0.67 {
            (fuzz::normal_f64(fuzz::cycle_n(seed, 2)).abs() * 4.0) as u32
        } else {
            let seed2 = fuzz::cycle_n(seed, 2);
            (fuzz::uniform_f64(seed2, fuzz::cycle(seed2)) * 30.0) as u32 + 10
        };
        let cap = capacity + extra_cap;
        let mut s = Segment::with_capacity_internal(cap);
        let mut seed = fuzz::cycle_n(seed, 4);
        let should_zero = (seed.count_ones() & 1) == 0;
        let fill_description = if should_zero { "zero" } else { "random" };
        if should_zero {
            for i in 1..cap {
                s.line[i as usize] = 0.into();
            }
        } else {
            for i in 1..cap {
                s.line[i as usize] = seed.into();
                seed = fuzz::cycle(seed);
            }
        }

        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'requested_capacity': {c}, 'given_capacity': {g}, 'filled_with': {f}{tail}",
                          n = "memory::segment::Segment", e = "create", s = s.line.line as usize,
                          c = capacity, g = cap, f = fill_description, tail = log_tail));
        s
    }

    fn free_internal(s: Segment) {
        unsafe {
            let cap = s.capacity();
            let v: Vec<Unit> =
                Vec::from_raw_parts(Unit::from(s).into(), 0,
                                    cap as usize);
            mem::drop(v);
        }
    }

    #[cfg(not(test))]
    pub fn free(s: Segment) {
        Segment::free_internal(s)
    }

    #[cfg(test)]
    pub fn free(s: Segment) {
        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}]",
                          n = "memory::segment::Segment", e = "free",
                          s = s.line.line as usize, c = s.capacity()));
        Segment::free_internal(s)
    }

    pub fn capacity(&self) -> u32 {
        Anchor::from(self.line[0]).capacity()
    }

    fn is_aliased_internal(&self) -> bool {
        Anchor::from(self.line[0]).is_aliased()
    }

    #[cfg(not(test))]
    pub fn is_aliased(&self) -> bool {
        self.is_aliased_internal()
    }

    #[cfg(test)]
    pub fn is_aliased(&self) -> bool {
        // spurious true returns
        let (x, log_tail) = fuzz::next_random();
        let should_true= ((x ^ (x >> 32)) & 0x7) == 0x7;
        let real_ret = self.is_aliased_internal();
        let intended_ret = real_ret || should_true;
        let return_description = if should_true && !real_ret { "spurious_true" }
        else {
            if real_ret { "true" } else { "false" }
        };

        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}, 'return': {r}{tail}",
                          n = "memory::segment::Segment", e = "is_aliased",
                          s = self.line.line as usize, c = self.capacity(),
                          r = return_description, tail = log_tail));
        intended_ret
    }

    fn alias_internal(&mut self) -> Anchor {
        // TODO CAS
        let a: usize = self.line[0].into();
        self.line[0] = (a + 1).into();
        Unit::from(a).into()
    }

    #[cfg(not(test))]
    pub fn alias(&mut self) {
        self.alias_internal();
    }

    #[cfg(test)]
    pub fn alias(&mut self) {
        let a = self.alias_internal();
        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}, 'alias_count': {ac}]",
                          n = "memory::segment::Segment", e = "alias",
                          s = self.line.line as usize, c = self.capacity(), ac = a.aliases()));
    }

    fn unalias_internal(&mut self) -> u32 {
        // TODO CAS
        let a: usize = self.line[0].into();
        self.line[0] = (a - 1).into();
        Anchor::from(self.line[0]).aliases()
    }

    #[cfg(not(test))]
    pub fn unalias(&mut self) -> u32 {
        self.unalias_internal()
    }

    #[cfg(test)]
    pub fn unalias(&mut self) -> u32 {
        let alias_count = self.unalias_internal();
        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}, 'alias_count': {ac}]",
                          n = "memory::segment::Segment", e = "unalias",
                          s = self.line.line as usize, c = self.capacity(), ac = alias_count));
        alias_count
    }

    pub fn line_with_offset(&self, offset: u32) -> AnchoredLine {
        AnchoredLine { base: *self, offset: offset as usize }
    }
}

#[derive(Copy, Clone)]
pub struct AnchoredLine {
    pub base: Segment,
    pub offset: usize,
}

impl Index<usize> for AnchoredLine {
    type Output = Unit;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.base, self.offset + index)
    }
}

impl IndexMut<usize> for AnchoredLine {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.base, self.offset + index)
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

    #[cfg(test)]
    fn index(&self, index: usize) -> &Self::Output {
        let cap = self.capacity() as usize;
        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}, 'index': {i}]",
                          n = "memory::segment::Segment", e = "index",
                          s = self.line.line as usize, c = cap, i = index));
        if index >= cap {
            panic!("Indexing {} outside Segment of capacity {}!", index, cap);
        }
        &self.line[index]
    }
    #[cfg(not(test))]
    fn index(&self, index: usize) -> &Self::Output {
        &self.line[index]
    }
}

impl IndexMut<usize> for Segment {
    #[cfg(test)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let cap = self.capacity() as usize;
        fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'capacity': {c}, 'index': {i}]",
                          n = "memory::segment::Segment", e = "index_mut",
                          s = self.line.line as usize, c = cap, i = index));
        if index >= cap || index == 0 {
            panic!("Indexing {} outside Segment of capacity {}.", index, cap);
        }
        if Anchor::from(self.line[0]).is_aliased() {
            panic!("Mut indexing {} in aliased Segment.", index);
        }
        &mut self.line[index]
    }
    #[cfg(not(test))]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.line[index]
    }
}

impl Index<u32> for Segment {
    type Output = Unit;

    fn index(&self, index: u32) -> &Self::Output {
        Index::index(self, index as usize)
    }
}

impl IndexMut<u32> for Segment {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        IndexMut::index_mut(self, index as usize)
    }
}

impl Index<i32> for Segment {
    type Output = Unit;

    fn index(&self, index: i32) -> &Self::Output {
        Index::index(self, index as usize)
    }
}

impl IndexMut<i32> for Segment {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        IndexMut::index_mut(self, index as usize)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
