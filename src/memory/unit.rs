//! A unit of memory.

/// A Unit is one processor word. Here, 64 bits.
#[cfg(target_arch = "x86_64")]
#[derive(Debug, Copy, Clone)]
pub struct Unit {
    word: usize,
}

use super::segment::Segment;
use super::anchor::{Anchor, AnchorLine};

impl From<Anchor> for Unit {
    fn from(a: Anchor) -> Self {
        a.unit
    }
}

impl Into<Anchor> for Unit {
    fn into(self) -> Anchor {
        Anchor { unit: self }
    }
}

impl From<AnchorLine> for Unit {
    fn from(al: AnchorLine) -> Self {
        al.line.into()
    }
}

impl Into<AnchorLine> for Unit {
    fn into(self) -> AnchorLine {
        AnchorLine { line: self.into() }
    }
}

impl From<Segment> for Unit {
    fn from(s: Segment) -> Self {
        s.line.into()
    }
}

impl Into<Segment> for Unit {
    fn into(self) -> Segment {
        Segment { line: self.into() }
    }
}

impl Unit {
    pub fn is_even(&self) -> bool {
        self.word & 0x01 == 0
    }

    // high zeros?
    // break into fields
    // bit manipulation needed to decode vector header
}


// Conversions around primitives
impl From<usize> for Unit {
    fn from(x: usize) -> Self {
        Unit { word: x }
    }
}

impl Into<usize> for Unit {
    fn into(self) -> usize {
        self.word
    }
}

impl From<isize> for Unit {
    fn from(x: isize) -> Self {
        Unit { word: x as usize }
    }
}

impl Into<isize> for Unit {
    fn into(self) -> isize {
        self.word as isize
    }
}

impl From<u64> for Unit {
    fn from(x: u64) -> Self {
        Unit { word: x as usize }
    }
}

impl Into<u64> for Unit {
    fn into(self) -> u64 {
        self.word as u64
    }
}

impl From<i64> for Unit {
    fn from(x: i64) -> Self {
        Unit { word: x as usize }
    }
}

impl Into<i64> for Unit {
    fn into(self) -> i64 {
        self.word as i64
    }
}

impl From<u32> for Unit {
    fn from(x: u32) -> Self {
        Unit { word: x as usize }
    }
}

impl Into<u32> for Unit {
    fn into(self) -> u32 {
        self.word as u32
    }
}

impl From<i32> for Unit {
    fn from(x: i32) -> Self {
        Unit { word: x as usize }
    }
}

impl Into<i32> for Unit {
    fn into(self) -> i32 {
        self.word as i32
    }
}

fn f64_into_u64(f: f64) -> u64 {
    use std::mem::transmute;
    unsafe {
        transmute(f)
    }
}
fn f64_from_u64(f: u64) -> f64 {
    use std::mem::transmute;
    unsafe {
        transmute(f)
    }
}

impl From<f64> for Unit {
    fn from(x: f64) -> Self {
        Unit { word: f64_into_u64(x) as usize }
    }
}

impl Into<f64> for Unit {
    fn into(self) -> f64 {
        f64_from_u64(self.word as u64)
    }
}

fn f32_into_u32(f: f32) -> u32 {
    use std::mem::transmute;
    unsafe {
        transmute(f)
    }
}
fn f32_from_u32(f: u32) -> f32 {
    use std::mem::transmute;
    unsafe {
        transmute(f)
    }
}

impl From<f32> for Unit {
    fn from(x: f32) -> Self {
        Unit { word: f32_into_u32(x) as usize }
    }
}

impl Into<f32> for Unit {
    fn into(self) -> f32 {
        f32_from_u32(self.word as u32)
    }
}

impl<T> From<*const T> for Unit {
    fn from(ptr: *const T) -> Self {
        Unit { word: ptr as usize }
    }
}

impl<T> Into<*const T> for Unit {
    fn into(self) -> *const T {
        self.word as *const T
    }
}

impl<T> From<*mut T> for Unit {
    fn from(ptr: *mut T) -> Self {
        Unit { word: ptr as usize }
    }
}

impl<T> Into<*mut T> for Unit {
    fn into(self) -> *mut T {
        self.word as *mut T
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_ptr() {
        let x = &5;
        let x_raw = x as *const i32;
        let u = Unit::from(x_raw);
        let xx: *const i32 = u.into();
        assert_eq!(x_raw, xx)
    }
}
