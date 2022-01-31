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
        let anchored = {
            let mut unanchored = unanchored_new(cap);
            unanchored.anchor_line[0] = Anchor::for_capacity(cap).into();
            unanchored
        };
        //println!("New segment({:?}) capacity {}", anchored.line().unit(), cap);
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
        //println!("Free segment({:?}), capacity {}", s.line().unit(), s.capacity());
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
        if cfg!(feature = "anchor_non_atomic") {
            let real_ret = self.anchor_line[0].anchor().is_aliased();
            real_ret
        } else {
            use std::sync::atomic::{AtomicUsize, Ordering};
            let ptr = self.anchor_line.star() as *const usize as *const AtomicUsize;
            let curr = unsafe { (&*ptr).load(Ordering::SeqCst) };
            let real_ret = Unit::from(curr).anchor().is_aliased();
            if real_ret {
                //use memory::schedule;
                //schedule::step();
            }
            real_ret
        }
        /*if cfg!(feature = "fuzz_segment_spurious_aliased") {
            use random::fuzz;
            let (seed, _log_tail) = fuzz::next_random();
            let spurious = ((seed ^ (seed >> 32)) & 0x7) == 0x7;
            real_ret || spurious
        } else {
            real_ret
        }*/
    }
    pub fn alias(&self) {
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.aliased();
            let mut x = *self;
            x.anchor_line[0] = new_a.into();
        } else {
            use std::sync::atomic::{AtomicUsize, Ordering};
            let ptr = self.anchor_line.star() as *const usize as *const AtomicUsize;
            let prev = unsafe { (&*ptr).fetch_add(1, Ordering::SeqCst) };
            let _ = Unit::from(prev).anchor().aliased(); // Alerts on overflow
        }
    }
    pub fn unalias(&self) -> u32 {
        if cfg!(feature = "anchor_non_atomic") {
            let a: Anchor = self.anchor_line[0].into();
            let new_a = a.unaliased();
            let mut x = *self;
            x.anchor_line[0] = new_a.into();
            new_a.aliases()
        } else {
            use std::sync::atomic::{AtomicUsize, Ordering};
            let ptr = self.anchor_line.star() as *const usize as *const AtomicUsize;
            let prev = unsafe { (&*ptr).fetch_sub(1, Ordering::SeqCst) };
            let new_anchor = Unit::from(prev).anchor().unaliased();
            new_anchor.aliases()
        }
    }
    pub fn unalias_expect_nonzero(&self) {
        if self.unalias() == 0 {
            panic!("Blind unalias op inappropriately used as final unalias")
        }
    }

    // TODO  prefer set to index assignment operator
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
                "segment_magic: casting to Segment failed, magic not found @ 0x{:016X} (found {})",
                line.unit().u(), line.offset(-1)[0].u());
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
    count_new(raw_cap);
    let v: Vec<Unit> = Vec::with_capacity(raw_cap as usize);
    let ptr = v.as_ptr();
    mem::forget(v);
    //println!("alloc({}) -> {:?}", raw_cap, Unit::from(ptr));
    Unit::from(ptr).into()
}

pub fn dealloc(line: Line, raw_cap: u32) {
    //println!("dealloc({:?}, {})", line, raw_cap);
    count_free(raw_cap);
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
        let v: Vec<Unit> =
            Vec::from_raw_parts(line.line as *mut Unit, 0, raw_cap as usize);
        mem::drop(v);
    }
}

impl Index<u32> for Segment {
    type Output = Unit;
    fn index(&self, index: u32) -> &Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
        assert!(index < self.capacity(),
                "segment_bounds ({:?}): accessing index = {}, segment capacity = {}.",
                self.anchor_line.unit(), index, self.capacity());
        //println!("Segment Index({:?}:{})[{}]", self.anchor_line.unit(), self.capacity(), index);
        &self.anchor_line[1 + index]
    }
}
impl IndexMut<u32> for Segment {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        #[cfg(any(test, feature = "segment_bounds"))]
        assert!(index < self.capacity(),
                "segment_bounds: writing index = {}, segment capacity = {}.",
                index, self.capacity());
        #[cfg(any(test, feature = "segment_mut"))]
        assert_eq!(self.anchor().aliases(), 1,
                "segment_mut: writing index = {}, segment aliases = {}.",
                index, self.anchor().aliases());
        //println!("Segment IndexMut({:?}:{})[{}]", self.anchor_line.unit(), self.capacity(), index);
        &mut self.anchor_line[1 + index]
    }
}

// TODO collect event counts and mem units totals
//  Map of segment sizes (in units) to segment count allocated and freed

#[derive(Copy, Clone, Debug)]
pub struct Usage {
    pub new_count: u64,
    pub new_units: u64,
    pub free_count: u64,
    pub free_units: u64,
}
impl Usage {
    pub fn new() -> Usage {
        Usage { new_count: 0, new_units: 0, free_count: 0, free_units: 0 }
    }
    pub fn add(mut self, other: &Usage) -> Usage {
        self.new_count += other.new_count;
        self.new_units += other.new_units;
        self.free_count += other.free_count;
        self.free_units += other.free_units;
        self
    }
}
use std::cell::Cell;
thread_local! {
    pub static USAGE: Cell<Usage> = Cell::new(Usage::new());
}
pub fn usage() -> Usage { USAGE.with(|c| c.get()) }
pub fn set_usage(u: Usage) { USAGE.with(|c| c.set(u)) }
pub fn count_new(capacity: u32) {
    if cfg!(any(test, feature = "segment_counts")) {
        let mut u = usage();
        u.new_count += 1;
        u.new_units += capacity as u64;
        set_usage(u);
    }
}
pub fn count_free(capacity: u32) {
    if cfg!(any(test, feature = "segment_counts")) {
        let mut u = usage();
        u.free_count += 1;
        u.free_units += capacity as u64;
        set_usage(u);
    }
}
pub fn new_free_counts() -> (u64, u64) {
    let u = usage();
    (u.new_count, u.free_count)
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

