// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;

// TODO make post a u64, don't encapsulate this fact.
// Necessitates switch in 32-bit word machines, store and rehydrate guide as two Units

#[derive(Copy, Clone)]
pub struct Guide {
    pub post: Unit,
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
        Guide { post: Unit::from(0) }
    }

    pub fn count(&self) -> u32 {
        let x: u64 = self.post.into();
        let large_count = (x >> 53) & 1;
        let field_width = (1u64 << large_count) << 4;
        let mask = !(!0u64 << field_width);
        (x & mask) as u32
    }

    pub fn has_meta(&self) -> bool {
        let x: u64 = self.post.into();
        let meta_bit = (x >> 54) & 1;
        meta_bit == 1
    }

    pub fn meta_gap(&self) -> u32 {
        let x: u64 = self.post.into();
        let large_count = (x >> 53) & 1;
        large_count as u32
    }

    pub fn has_hash(&self) -> bool {
        let x: u64 = self.post.into();
        let hash_bit = (x >> 55) & 1;
        hash_bit == 1
    }

    pub fn inc(&self) -> Guide {
        let x: u64 = self.post.into();
        Unit::from(x + 1).into()
    }

    pub fn prism_to_anchor_gap(&self) -> u32 {
        let x: u64 = self.post.into();
        (x >> 56) as u32
    }

    pub fn guide_to_root_gap(&self) -> u32 {
        let x: u64 = self.post.into();
        ((x >> 48) & 0b111) as u32
    }

    pub fn inc_guide_to_root_gap(&self) -> Guide {
        let x: u64 = self.post.into();
        Unit::from(x + (1u64 << 48)).into()
    }
}

impl From<Unit> for Guide {
    fn from(u: Unit) -> Self {
        Guide { post: u }
    }
}

impl Into<Unit> for Guide {
    fn into(self) -> Unit {
        self.post
    }
}
