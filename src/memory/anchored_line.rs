// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::ops::{Index, IndexMut};
use memory::*;

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
        let mut m = *self;
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

    pub fn has_index(&self, index: i32) -> bool {
        let i = ((self.index as i32) + index) as u32;
        self.seg.has_index(i)
    }

    pub fn with_seg(&self, seg: Segment) -> AnchoredLine {
        AnchoredLine { seg, index: self.index }
    }

    pub fn range(&self, length: u32) -> AnchoredRange {
        AnchoredRange::new(self.seg, self.index..(self.index + length))
    }

    pub fn line(&self) -> Line {
        self.seg.anchor_line.offset(self.index as isize + 1)
    }

    pub fn span(&self, width: u32) -> AnchoredRange {
        AnchoredRange::new(self.seg, self.index..(self.index + width))
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
