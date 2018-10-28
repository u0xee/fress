// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;

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

/// The Guide structure is hydrated from its in-memory representation, 64 bits in length.
/// The top 32 bits contain the hash, the bottom contain the collection's count.
/// Also in the top, two booleans represent the presence of meta, and the prism's index.
/// The two lowest order bits are not part of the hash, they are the booleans.
/// So a collection's hash will always end in two zero bits.
/// The highest order two bits of the bottom 32 bits also represent two booleans:
/// is the collection a set, and does the collection have a unit of information besides
/// its root elements?
/// So a collection's count resides in the 30 lowest order bits.

/// ```
/// Top 32 bits  [    Hash (30)   | Prism Index | Meta? ]
/// Bottom bits  [ Set? | No unit? |     Count (30)     ]
/// ```
///

#[derive(Copy, Clone)]
pub struct Guide {
    pub count: u32,
    pub hash: u32,
    pub prism_idx: u32,
    pub root_idx: u32,
    pub has_meta: bool,
    pub pre_root_unit: bool,
    pub is_set: bool, // u32
}

impl Guide {
    pub fn hydrate(prism: Line) -> Guide {
        if cfg!(target_pointer_width = "32") {
            Guide::hydrate_top_bot(prism, prism[1].into(), prism[2].into())
        } else {
            let g: u64 = prism[1].into();
            Guide::hydrate_top_bot(prism, (g >> 32).into(), g.into())
        }
    }

    pub fn hydrate_top_bot(prism: Line, top: u32, bot: u32) -> Guide {
        let hash = top & !0x3;
        let prism_zero = (top & 0x2) == 0;
        let has_meta_bit = (top & 0x1);

        let count = bot & !(0x3 << 30);
        let is_set = ((bot >> 30) & 0x2) != 0;
        let pre_root_unit_bit = ((bot >> 30) & 0x1);

        let prism_idx: u32 = if prism_zero { 0 } else {
            prism.offset(-1)[0].into()
        };
        let root_gap = has_meta_bit + pre_root_unit_bit;
        let root_idx = prism_idx + 2 + root_gap +
            if cfg!(target_pointer_width = "32") { 1 } else { 0 };
        Guide {
            count, hash, prism_idx, root_idx, is_set,
            has_meta: has_meta_bit != 0,
            pre_root_unit: pre_root_unit_bit != 0,
        }
    }

    pub fn store(&self, prism: AnchoredLine) {
        let top: u32 = 0;
        let bot: u32 = 0;
        if cfg!(target_pointer_width = "32") {
            prism[1] = top.into();
            prism[2] = bot.into();
        } else {
            let g: u64 = ((top as u64) << 32) | (bot as u64);
            prism[1] = g.into();
        }
    }

    pub fn new() -> Guide {
        Guide {
            count: 0, hash: 0, prism_idx: 0,
            root_idx: 2 + if cfg!(target_pointer_width = "32") { 1 } else { 0 },
            has_meta: false, pre_root_unit: false, is_set: false
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }

    pub fn has_meta(&self) -> bool {
        self.has_meta
    }

    // TODO

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
