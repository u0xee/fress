// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! A unit of memory.
use std::fmt;
use std::mem::transmute;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use memory::*;
use handle::Handle;

/// A Unit is one processor word. Here, 64 or 32 bits.

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Unit {
    pub word: usize,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:#X}", self.word) }
}

impl Unit {
    pub fn width() -> u32 { if cfg!(target_pointer_width = "32") { 32 } else { 64 } }
    pub fn bytes() -> u32 { if cfg!(target_pointer_width = "32") { 4 } else { 8 } }
    pub fn zero() -> Unit { Unit::from(0usize) }
    pub fn is_even(&self) -> bool { self.word & 0x01 == 0 }
    pub fn u(&self) -> usize { self.word as usize }
    pub fn i(&self) -> isize { self.word as isize }
    pub fn u64(&self) -> u64 { self.word as u64 }
    pub fn u32(&self) -> u32 { self.word as u32 }
    pub fn anchor(self) -> Anchor { Anchor::from(self) }
    pub fn line(self) -> Line { Line::from(self) }
    pub fn segment(self) -> Segment { Segment::from(self) }
    pub fn handle(self) -> Handle { Handle::from(self) }
}

impl From<usize> for Unit { fn from(x: usize) -> Self { Unit { word: x } } }
impl Into<usize> for Unit { fn into(self) -> usize { self.word } }
impl From<isize> for Unit { fn from(x: isize) -> Self { Unit { word: x as usize } } }
impl Into<isize> for Unit { fn into(self) -> isize { self.word as isize } }
impl From<u64> for Unit { fn from(x: u64) -> Self { Unit { word: x as usize } } }
impl Into<u64> for Unit { fn into(self) -> u64 { self.word as u64 } }
impl From<i64> for Unit { fn from(x: i64) -> Self { Unit { word: x as usize } } }
impl Into<i64> for Unit { fn into(self) -> i64 { self.word as i64 } }
impl From<u32> for Unit { fn from(x: u32) -> Self { Unit { word: x as usize } } }
impl Into<u32> for Unit { fn into(self) -> u32 { self.word as u32 } }
impl From<u16> for Unit { fn from(x: u16) -> Self { Unit { word: x as usize } } }
impl Into<u16> for Unit { fn into(self) -> u16 { self.word as u16 } }
impl From<i32> for Unit { fn from(x: i32) -> Self { Unit { word: x as usize } } }
impl Into<i32> for Unit { fn into(self) -> i32 { self.word as i32 } }
pub fn f64_into_u64(f: f64) -> u64 { unsafe { transmute(f) } }
pub fn f64_from_u64(f: u64) -> f64 { unsafe { transmute(f) } }
pub fn f32_into_u32(f: f32) -> u32 { unsafe { transmute(f) } }
pub fn f32_from_u32(f: u32) -> f32 { unsafe { transmute(f) } }
impl From<f64> for Unit { fn from(x: f64) -> Self { Unit { word: f64_into_u64(x) as usize } } }
impl Into<f64> for Unit { fn into(self) -> f64 { f64_from_u64(self.word as u64) } }
impl From<f32> for Unit { fn from(x: f32) -> Self { Unit { word: f32_into_u32(x) as usize } } }
impl Into<f32> for Unit { fn into(self) -> f32 { f32_from_u32(self.word as u32) } }
impl<T> From<*const T> for Unit { fn from(ptr: *const T) -> Self { Unit { word: ptr as usize } } }
impl<T> Into<*const T> for Unit { fn into(self) -> *const T { self.word as *const T } }
impl<T> From<*mut T> for Unit { fn from(ptr: *mut T) -> Self { Unit { word: ptr as usize } } }
impl<T> Into<*mut T> for Unit { fn into(self) -> *mut T { self.word as *mut T } }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ptr_roundtrip() {
        let x = &5;
        let x_raw = x as *const i32;
        assert_eq!(x_raw, Unit::from(x_raw).into())
    }
    #[test]
    fn float_roundtrip() {
        let x = 5f32;
        assert_eq!(x, Unit::from(x).into());
        if cfg!(target_pointer_width = "64") {
            let x = 5f64;
            assert_eq!(x, Unit::from(x).into());
        }
    }
    #[test]
    fn int_roundtrip() {
        let x = -5i32;
        assert_eq!(x, Unit::from(x).into());
        let x = 5u32;
        assert_eq!(x, Unit::from(x).into());
    }
}
