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
//! A segment, concretely, is a line to its anchor element, which sits right before the first
//! content element. The anchor contains information about
//! its segment, such as the number of content units it contains.
//! The anchor allows one or more threads to read information from the segment, sharing
//! the responsibility to return the segment to the memory pool, when no longer needed.
//! The anchor supports this by counting the number of aliases to its segment.

use std::mem;
use std::fmt;
use std::ops::{Index, IndexMut};
use fuzz;
use memory::*;

#[derive(Copy, Clone)]
pub struct Segment {
    pub anchor_line: Line,
}

impl Segment {
    pub fn new(content_cap: u32) -> Segment {
        trace::new_BEGIN(content_cap);
        let mut cap = content_cap;
        #[cfg(any(test, feature = "fuzz_segment_extra_cap"))]
            { cap += extra_cap(); }
        let mut unanchored = unanchored_new(cap);
        unanchored.anchor_line[0] = Anchor::for_capacity(cap).into();
        let mut anchored = unanchored;
        #[cfg(any(test, feature = "segment_clear"))]
            for i in 0..cap {
                anchored.anchor_line[1 + i as usize] = 0.into();
            }
        #[cfg(any(test, feature = "fuzz_segment_random_content"))]
            random_content(anchored, cap);
        trace::new_END(anchored, cap);
        anchored
    }

    pub fn free(s: Segment) {
        trace::free_BEGIN(s);
        let a = Anchor::from(s.anchor_line[0]);
        #[cfg(any(test, feature = "segment_free"))]
            assert_eq!(a.aliases(), 0);
        if cfg!(any(test, feature = "segment_magic")) {
            dealloc(s.anchor_line.offset(-1 as isize), a.capacity() + 2);
        } else {
            dealloc(s.anchor_line, a.capacity() + 1);
        }
        trace::free_END(s.anchor_line);
    }

    pub fn capacity(&self) -> u32 {
        self.anchor_line[0].anchor().capacity()
    }

    pub fn is_aliased(&self) -> bool {
        let real_ret = self.line[0].anchor().is_aliased();
        if cfg!(any(test, feature = "fuzz_segment_spurious_aliased")) {
            use fuzz;
            let (seed, log_tail) = fuzz::next_random();
            let spurious = (seed ^ (seed >> 32) & 0x7) == 0x7;
            real_ret || spurious
        } else {
            real_ret
        }
    }

    pub fn alias(&self) {
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.aliased();
            let mut  x = self;
            x.anchor_line[0] = new_a.into();
        } else {
            unimplemented!()
        }
    }

    pub fn unalias(&self) -> u32 {
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.unaliased();
            let mut x = self;
            x.anchor_line[0] = new_a.into();
            new_a.aliases()
        } else {
            unimplemented!()
        }
    }

    pub fn get(&self, index: u32) -> Unit {
        self[index]
    }

    pub fn set(&self, index: u32, x: Unit) {
        let mut m = self;
        m[index] = x;
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor_line[0].anchor()
    }

    pub fn unit(&self) -> Unit {
        self.anchor_line.unit()
    }

    pub fn line_at(&self, base: u32) -> AnchoredLine {
        AnchoredLine::new(self, base)
    }

    pub fn line(&self) -> Line {
        self.anchor_line
    }
}

impl From<Unit> for Segment {
    fn from(unit: Unit) -> Self {
        Segment::from(unit.line())
    }
}

impl From<Line> for Segment {
    fn from(line: Line) -> Self {
        if cfg!(any(test, feature = "segment_magic")) {
            if line.offset(-1)[0] != 0xCAFEBABE.into() {
                panic!("Casting to Segment failed, magic not found")
            }
            Segment { anchor_line: line }
        } else {
            Segment { anchor_line: line }
        }
    }
}

pub fn unanchored_new(cap: u32) -> Segment {
    if cfg!(any(test, feature = "segment_magic")) {
        let mut line = alloc(cap + 2);
        line[0] = 0xCAFEBABE.into();
        Segment { anchor_line: line.offset(1 as isize) }
    } else {
        Segment { anchor_line: alloc(cap + 1) }
    }
}

pub fn alloc(raw_cap: u32) -> Line {
    let v: Vec<Unit> = Vec::with_capacity(raw_cap as usize);
    let ptr = v.as_ptr();
    mem::forget(v);
    Unit::from(ptr).into()
}

pub fn dealloc(mut line: Line, raw_cap: u32) {
    #[cfg(any(test, feature = "segment_clear"))]
        for i in 0..raw_cap {
            line[i as usize] = 0.into();
        }
    unsafe {
        let v: Vec<Unit> = Vec::from_raw_parts(line.line as *mut Unit,
                                               0, raw_cap as usize);
        mem::drop(v);
    }
}

#[cfg(any(test, feature = "fuzz_segment_extra_cap"))]
pub fn extra_cap() -> u32 {
    use fuzz;
    let (seed, log_tail) = fuzz::next_random();
    let p = fuzz::uniform_f64(seed, fuzz::cycle(seed));
    // fuzz::log(format!("['namespace': {n}, 'event_name': {e}, 'segment': {s}, 'cap': {c}{tail}",
    if p < 0.67 {
        (fuzz::normal_f64(fuzz::cycle_n(seed, 2)).abs() * 4.0) as u32
    } else {
        let seed2 = fuzz::cycle_n(seed, 2);
        (fuzz::uniform_f64(seed2, fuzz::cycle(seed2)) * 30.0) as u32 + 10
    }
}

#[cfg(any(test, feature = "fuzz_segment_random_content"))]
pub fn random_content(mut s: Segment, cap: u32) {
    use fuzz;
    let (mut seed, log_tail) = fuzz::next_random();
    for i in 0..cap {
        s.anchor_line[1 + i] = seed.into();
        seed = fuzz::cycle(seed);
    }
    // fuzz::log
}

impl Index<u32> for Segment {
    type Output = Unit;

    fn index(&self, index: u32) -> &Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
            {
                if index >= self.capacity() {
                    panic!("Indexing {} outside Segment of capacity {}.", index, self.capacity());
                }
            }
        &self.anchor_line[1 + index]
    }
}

impl IndexMut<u32> for Segment {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
            {
                if index >= self.capacity() {
                    panic!("Indexing {} outside Segment of capacity {}.", index, self.capacity());
                }
            }
        #[cfg(any(test, feature = "segment_mut"))]
            {
                if self.anchor().is_aliased() {
                    panic!("Mut indexing {} in aliased Segment.", index);
                }
            }
        &mut self.anchor_line[1 + index]
    }
}

#[derive(Copy, Clone)]
pub struct AnchoredLine {
    pub seg: Segment,
    pub index: u32,
}

impl AnchoredLine {
    pub fn new(seg: Segment, index: u32) -> AnchoredLine {
        AnchoredLine { seg, index }
    }

    pub fn get(&self, index: i32) -> Unit {
        self[index]
    }

    pub fn set(&self, index: i32, x: Unit) {
        let mut m = self;
        m[index] = x;
    }

    pub fn segment(&self) -> Segment {
        self.seg
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn offset(&self, offset: i32) -> AnchoredLine {
        AnchoredLine { seg: self.seg, index: ((self.index as i32) + offset) as u32 }
    }

    pub fn line(&self) -> Line {
        self.seg.anchor_line.offset(self.index as isize + 1)
    }
}

impl Index<i32> for AnchoredLine {
    type Output = Unit;

    fn index(&self, index: i32) -> &Self::Output {
        Index::index(&self.seg, ((self.index as i32) + index) as u32)
    }
}

impl IndexMut<i32> for AnchoredLine {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.seg, ((self.index as i32) + index) as u32)
    }
}

pub mod trace {
    use super::*;
    pub fn new_BEGIN(content_cap: u32) { }
    pub fn new_END(s: Segment, content_cap: u32) { }
    pub fn free_BEGIN(s: Segment) { }
    pub fn free_END(s: Line) { }
}
