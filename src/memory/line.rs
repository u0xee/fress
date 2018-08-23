// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use memory::segment::Segment;
use std::ops::{Index, IndexMut};

pub struct Line {
    pub line: *const Unit,
}

impl Line {
    pub fn offset(&self, x: isize) -> Line {
        Line { line: unsafe { self.line.offset(x) } }
    }
}

impl From<Unit> for Line {
    fn from(u: Unit) -> Self {
        Line { line: u.into() }
    }
}

impl Into<Unit> for Line {
    fn into(self) -> Unit {
        Unit::from(self.line)
    }
}

impl From<Segment> for Line {
    fn from(seg: Segment) -> Self {
        seg.line
    }
}

impl Index<usize> for Line {
    type Output = Unit;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &*self.line.offset(index as isize)
        }
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *(self.line as *mut Unit).offset(index as isize)
        }
    }
}
