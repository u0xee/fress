// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::ops::{Index, IndexMut};
use memory::*;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub line: *const Unit,
}

impl Line {
    pub fn unit(self) -> Unit {
        Unit::from(self.line)
    }
    pub fn segment(self) -> Segment {
        Segment::from(self)
    }
    pub fn offset(&self, x: isize) -> Line {
        Line { line: unsafe { self.line.offset(x) } }
    }
    pub fn star(self) -> *const Unit {
        self.line
    }
}

impl From<Unit> for Line {
    fn from(u: Unit) -> Self {
        Line { line: u.into() }
    }
}

impl Index<u32> for Line {
    type Output = Unit;

    fn index(&self, index: u32) -> &Self::Output {
        unsafe {
            &*self.line.offset(index as isize)
        }
    }
}

impl IndexMut<u32> for Line {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        unsafe {
            &mut *(self.line as *mut Unit).offset(index as isize)
        }
    }
}
