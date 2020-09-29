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
use std::ops::{Index, IndexMut, Range};
use memory::*;

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub anchor_line: Line,
}

impl Segment {
    pub fn new(cap: u32) -> Segment {
        inc_new_count();
        let anchored = {
            let mut unanchored = unanchored_new(cap);
            unanchored.anchor_line[0] = Anchor::for_capacity(cap).into();
            unanchored
        };
        //log!("New segment, capacity {}, @ 0x{:016X}", cap, anchored.line().unit().u());
        #[cfg(any(test, feature = "segment_clear"))]
            {
                let mut anchored = anchored;
                for i in 0..cap {
                    anchored.anchor_line[1 + i] = 0.into();
                }
            }
        anchored
    }

    pub fn free(s: Segment) {
        inc_free_count();
        //log!("Free segment, capacity {}, @ 0x{:016X}", s.capacity(), s.line().unit().u());
        let a = Anchor::from(s.anchor_line[0]);
        #[cfg(any(test, feature = "segment_free"))]
            assert_eq!(a.aliases(), 0,
                       "segment_free: freeing segment with aliases = {}", a.aliases());
        if cfg!(any(test, feature = "segment_magic")) {
            dealloc(s.anchor_line.offset(-1 as isize), a.capacity() + 2);
        } else {
            dealloc(s.anchor_line, a.capacity() + 1);
        }
    }

    pub fn capacity(&self) -> u32 { self.anchor_line[0].anchor().capacity() }

    pub fn is_aliased(&self) -> bool {
        let real_ret = self.anchor_line[0].anchor().is_aliased();
        if cfg!(feature = "fuzz_segment_spurious_aliased") {
            use random::fuzz;
            let (seed, _log_tail) = fuzz::next_random();
            let spurious = ((seed ^ (seed >> 32)) & 0x7) == 0x7;
            real_ret || spurious
        } else {
            real_ret
        }
    }

    pub fn alias(&self) {
        //log!("Segment alias");
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.aliased();
            let mut x = *self;
            x.anchor_line[0] = new_a.into();
        } else {
            unimplemented!()
        }
    }
    pub fn unalias(&self) -> u32 {
        //log!("Segment unalias");
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.unaliased();
            let mut x = *self;
            x.anchor_line[0] = new_a.into();
            new_a.aliases()
        } else {
            unimplemented!()
        }
    }

    pub fn get(&self, index: u32) -> Unit { self[index] }
    pub fn set(&self, index: u32, x: Unit) {
        let mut m = *self;
        m[index] = x;
    }
    pub fn has_index(&self, index: u32) -> bool { index < self.capacity() }
    pub fn anchor(&self) -> Anchor { self.anchor_line[0].anchor() }
    pub fn unit(&self) -> Unit { self.anchor_line.unit() }
    pub fn line_at(&self, base: u32) -> AnchoredLine { AnchoredLine::new(*self, base) }
    pub fn at(&self, range: Range<u32>) -> AnchoredRange { AnchoredRange::new(*self, range) }
    pub fn line(&self) -> Line { self.anchor_line }

    pub fn store_hash(&self, index: u32, x: Unit) {
        #[cfg(any(test, feature = "segment_bounds"))]
        assert!(index < self.capacity(),
                "segment_bounds: accessing index = {}, segment capacity = {}.",
                index, self.capacity());
        // In the normal write path, we would check that the segment alias count is one.
        // Here, the hash is computed on demand, so we may store to a shared segment,
        // in this case, on purpose.
        let mut m = *self;
        m.anchor_line[1 + index] = x;
    }
    pub fn carbon_copy(&self) -> Segment {
        let cap = self.capacity();
        let s = Segment::new(cap);
        self.at(0..cap).to(s);
        s
    }

    pub fn print_bits(&self) {
        let base = self.unit().u();
        eprintln!(" Segment {{ address: {:X}, aliases: {}, capacity: {},",
                 base, self.anchor().aliases(), self.capacity());
        let cap = self.capacity();
        let rows_of_four = ((cap - 1) / 4) + 1;
        for row in 0..rows_of_four {
            eprint!("  ");
            for i in 0..4 {
                let index = 4 * row + i;
                if index < cap {
                    let x = self.get(index).u();
                    let diff = base ^ x;
                    if diff.leading_zeros() > base.leading_zeros() {
                        let xx = (!0 >> diff.leading_zeros()) & x;
                        eprint!("{:2}: {:.>12X}  ", index, xx);
                    } else {
                        eprint!("{:2}: {:_>12X}  ", index, x);
                    }
                }
            }
            eprintln!();
        }
        eprintln!(" }}");
    }
}

impl From<Unit> for Segment {
    fn from(unit: Unit) -> Self { Segment::from(unit.line()) }
}
fn is_aligned(line: Line) -> bool {
    let mask = Unit::bytes() as usize - 1;
    line.unit().u() & mask == 0
}
fn has_magic(line: Line) -> bool {
    line.offset(-1)[0] == 0xCAFEBABEu32.into()
}
impl From<Line> for Segment {
    fn from(line: Line) -> Self {
        #[cfg(any(test, feature = "segment_null"))]
        assert_ne!(line.unit(), Unit::zero(),
                   "segment_null: null can't be used as a segment");
        #[cfg(any(test, feature = "segment_unaligned"))]
        assert!(is_aligned(line),
                "segment_unaligned: unaligned number can't be a segment, 0x{:016X}",
                line.unit().u());
        #[cfg(any(test, feature = "segment_magic"))]
        assert!(has_magic(line),
                "segment_magic: casting to Segment failed, magic not found @ 0x{:016X}",
                line.unit().u());
        Segment { anchor_line: line }
    }
}

pub fn unanchored_new(cap: u32) -> Segment {
    if cfg!(any(test, feature = "segment_magic")) {
        let mut line = alloc(cap + 2);
        line[0] = 0xCAFEBABEu32.into();
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

pub fn dealloc(line: Line, raw_cap: u32) {
    #[cfg(any(test, feature = "segment_erase"))]
        {
            let mut line = line;
            for i in 0..raw_cap {
                line[i] = 0.into();
            }
        }
    if cfg!(any(test, feature = "segment_fallow")) {
        /*do nothing, segment will lie fallow*/
    } else {
        recycle(line, raw_cap)
    }
}
pub fn recycle(line: Line, raw_cap: u32) {
    unsafe {
        let v: Vec<Unit> = Vec::from_raw_parts(line.line as *mut Unit,
                                               0, raw_cap as usize);
        mem::drop(v);
    }
}

impl Index<u32> for Segment {
    type Output = Unit;
    fn index(&self, index: u32) -> &Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
        assert!(index < self.capacity(),
                "segment_bounds: accessing index = {}, segment capacity = {}.",
                index, self.capacity());
        &self.anchor_line[1 + index]
    }
}
impl IndexMut<u32> for Segment {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
        assert!(index < self.capacity(),
                "segment_bounds: accessing index = {}, segment capacity = {}.",
                index, self.capacity());
        #[cfg(any(test, feature = "segment_mut"))]
        assert_eq!(self.anchor().aliases(), 1,
                "segment_mut: writing index = {}, segment aliases = {}.",
                index, self.anchor().aliases());
        &mut self.anchor_line[1 + index]
    }
}

// TODO collect event counts and mem units totals
//  Total allocated: segment count and units sum
//  Total freed: segment count and units sum
//  Map of segment sizes (in units) to segment count allocated and freed
use std::cell::Cell;
thread_local! {
    pub static NEW_COUNT: Cell<u64> = Cell::new(0);
    pub static FREE_COUNT: Cell<u64> = Cell::new(0);
}
pub fn inc_new_count() { NEW_COUNT.with(|c| c.set(c.get() + 1)) }
pub fn inc_free_count() { FREE_COUNT.with(|c| c.set(c.get() + 1)) }
pub fn new_free_counts() -> (u64, u64) {
    (NEW_COUNT.with(|c| c.get()), FREE_COUNT.with(|c| c.get()))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic(expected = "segment_bounds")]
    fn bounds_read() {
        let s = Segment::new(5);
        s[5];
    }
    #[test]
    #[should_panic(expected = "segment_bounds")]
    fn bounds_write() {
        let mut s = Segment::new(5);
        s[5] = 0.into();
    }
    #[test]
    #[should_panic(expected = "segment_mut")]
    fn aliased_write() {
        let mut s = Segment::new(5);
        s.alias();
        s[4] = 0.into()
    }
    #[test]
    #[should_panic(expected = "segment_magic")]
    fn magic_missing() {
        let s = Segment::new(5);
        let off = s.line().offset(1);
        let r = off.segment();
    }
    #[test]
    #[should_panic(expected = "segment_null")]
    fn null_line() {
        let u = Unit::from(0usize);
        let r = u.segment();
    }
    #[test]
    #[should_panic(expected = "segment_unaligned")]
    fn unaligned_line() {
        let u = Unit::from(0xABCDEusize);
        let r = u.segment();
    }
    #[test]
    #[should_panic(expected = "segment_free")]
    fn aliased_free() {
        let s = Segment::new(5);
        Segment::free(s);
    }
    #[test]
    fn zeroed() {
        let s = Segment::new(5);
        for i in 0..5 {
            assert_eq!(0, s[i].u());
        }
    }
    #[test]
    fn wiped() {
        let s = Segment::new(5);
        for i in 0..5 {
            s.set(i, 1.into());
        }
        let line = s.line().offset(-1 as isize);
        for i in 0..7 {
            assert_ne!(0, line[i].u());
        }
        s.unalias();
        Segment::free(s);
        for i in 0..7 {
            assert_eq!(0, line[i].u());
        }
    }
}

