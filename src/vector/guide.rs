// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;

// TODO store and read guide in 32-bit environment

#[derive(Copy, Clone)]
pub struct Guide {
    pub post: u64,
}

// Layout of guide unit, 64bits in bytes:
// A B H H | H H C C
// A byte is distance to anchor
// Four H bytes are for storing the hash
// Two C bytes for storing the count of the collection

// New B byte layout
// Info byte in bit fields:
// h? m? l? u | u ic ic ic
// hash present?
// meta present?
// large count?
// index of contents - 3 bits

// Count algorithm:
// Isolate the large-count-? bit
// Shift one (a constant) left by this bit
// Shift the result left by four (a constant)
// Negate 0u64 (like -1 in signed numbers), shift it left by the result above
// Negate this, producing a 32 or 16 bit wide mask
// Use the mask to isolate the count field, or splice into the count field

// fields: count, anchor_distance, hash?, hash, meta?, root_offset

impl Guide {
    pub fn new() -> Guide {
        Guide { post: 0u64 }
    }

    pub fn count(&self) -> u32 {
        let x: u64 = self.post;
        let large_count = (x >> 53) & 1;
        let field_width = (1u64 << large_count) << 4;
        let mask = !(!0u64 << field_width);
        (x & mask) as u32
    }

    pub fn has_meta(&self) -> bool {
        let x: u64 = self.post;
        let meta_bit = (x >> 54) & 1;
        meta_bit == 1
    }

    pub fn with_meta(&self) -> Guide {
        let x: u64 = self.post;
        let meta_bit = (x >> 54) & 1;
        (x | (1 << 54)).into()
    }

    pub fn meta_gap(&self) -> u32 {
        let x: u64 = self.post;
        let large_count = (x >> 53) & 1;
        let extra_guide_unit: u32 = if cfg!(target_pointer_width = "32") { 1 } else { 0 };
        (large_count as u32) + extra_guide_unit
    }

    pub fn has_hash(&self) -> bool {
        let x: u64 = self.post;
        let hash_bit = (x >> 55) & 1;
        hash_bit == 1
    }

    pub fn inc(&self) -> Guide {
        let x: u64 = self.post;
        (x + 1).into()
    }

    pub fn dec(&self) -> Guide {
        let x: u64 = self.post;
        (x - 1).into()
    }

    pub fn prism_to_anchor_gap(&self) -> u32 {
        let x: u64 = self.post;
        (x >> 56) as u32
    }

    pub fn with_anchor_gap_change(&self, delta: i32) -> Guide {
        let x: u64 = self.post;
        let anchor_gap = self.prism_to_anchor_gap();
        let new_gap = ((anchor_gap as i32) + delta) as u64;
        let mask = (1u64 << 56) - 1;
        let new_x = (x & mask) | (new_gap << 56);
        Unit::from(new_x).into()
    }

    pub fn guide_to_root_gap(&self) -> u32 {
        let x: u64 = self.post;
        ((x >> 48) & 0b111) as u32
    }

    pub fn inc_guide_to_root_gap(&self) -> Guide {
        let x: u64 = self.post;
        Unit::from(x + (1u64 << 48)).into()
    }

    pub fn with_root_gap_change(&self, delta: i32) -> Guide {
        let x: u64 = self.post;
        // TODO verify works
        Unit::from(x + ((delta as u64) << 48)).into()
    }
}

impl From<u64> for Guide {
    fn from(x: u64) -> Self {
        Guide { post: x }
    }
}

impl Into<u64> for Guide {
    fn into(self) -> u64 {
        self.post
    }
}

impl From<Unit> for Guide {
    fn from(u: Unit) -> Self {
        Guide { post: u.into() }
    }
}

impl Into<Unit> for Guide {
    fn into(self) -> Unit {
        Unit::from(self.post)
    }
}
