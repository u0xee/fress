// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;

/// The Guide structure is hydrated from its in-memory representation, 64 bits in length.
/// The top 32 bits contain the hash, the bottom 32 bits contain the collection's count.
/// Also in the top, two booleans represent the presence of a hash and of meta.
/// The two lowest order bits are not part of the hash, they are the booleans.
/// So a collection's hash will always end in two zero bits.
/// The highest order two bits of the bottom 32 bits also represent two booleans:
/// is the collection a set, and is the representation compact (no info unit).
/// So a collection's count resides in the 30 lowest order bits.

/// ```
/// Top 32 bits  [    Hash (30)    | Hash? | Meta? ]
/// Bottom bits  [ Set? | Compact? |   Count (30)  ]
/// ```
///

#[derive(Copy, Clone)]
pub struct Guide {
    pub hash: u32,
    pub has_hash_bit: u32,
    pub has_meta_bit: u32,

    pub is_set_bit: u32,
    pub is_compact_bit: u32,
    pub count: u32,

    pub prism: AnchoredLine,
    pub root: AnchoredLine,
}

impl Guide {
    pub fn set_hash(self, hash: u32) -> Guide {
        self.hash = hash & !0x3;
        self.has_hash_bit = 1;
        self
    }

    pub fn clear_hash(self) -> Guide {
        self.hash = 0;
        self.has_hash_bit = 0;
        self
    }

    pub fn has_hash(&self) -> bool {
        self.has_hash_bit == 1
    }

    pub fn set_meta(self) -> Guide {
        self.has_meta_bit = 1;
        self
    }

    pub fn clear_meta(self) -> Guide {
        self.has_meta_bit = 0;
        self
    }

    pub fn has_meta(&self) -> bool {
        self.has_meta_bit == 1
    }

    pub fn meta_line(&self) -> AnchoredLine {
        self.prism.offset(if cfg!(target_pointer_width = "32") { 3 } else { 2 })
    }

    pub fn clear_compact(self) -> Guide {
        self.is_compact_bit = 0;
        self
    }

    pub fn inc_count(self) -> Guide {
        self.count = self.count + 1;
        self
    }

    pub fn dec_count(self) -> Guide {
        self.count = self.count - 1;
        self
    }

    pub fn reroot(self) -> Guide {
        let root_offset = 1 /*prism*/ +
            if cfg!(target_pointer_width = "32") { 2 } else { 1 } /*guide*/ +
            self.has_meta_bit + (!self.is_compact_bit & 0x1);
        self.root = self.prism.offset(root_offset as i32);
        self
    }

    pub fn hydrate(prism: AnchoredLine) -> Guide {
        if cfg!(target_pointer_width = "32") {
            Guide::hydrate_top_bot(prism, prism[1].into(), prism[2].into())
        } else {
            let g: u64 = prism[1].into();
            Guide::hydrate_top_bot(prism, (g >> 32).into(), g.into())
        }
    }

    pub fn hydrate_top_bot(prism: AnchoredLine, top: u32, bot: u32) -> Guide {
        let hash = top & !0x3;
        let has_hash_bit = (top >> 1) & 0x1;
        let has_meta_bit = (top & 0x1);

        let count = bot & !(0x3 << 30);
        let is_set_bit = (bot >> 31) & 0x1;
        let is_compact_bit = (bot >> 30) & 0x1;

        let root_offset = 1 /*prism*/ +
            if cfg!(target_pointer_width = "32") { 2 } else { 1 } /*guide*/ +
            has_meta_bit + (!is_compact_bit & 0x1);
        let root = prism.offset(root_offset as i32);

        Guide { hash, has_hash_bit, has_meta_bit, count, is_set_bit, is_compact_bit, prism, root }
    }

    pub fn store(&self, prism: AnchoredLine) {
        let top: u32 = self.hash | (self.has_hash_bit << 1) | self.has_meta_bit;
        let bot: u32 = (self.is_set_bit << 31) | (self.is_compact_bit << 30) | self.count;
        if cfg!(target_pointer_width = "32") {
            prism[1] = top.into();
            prism[2] = bot.into();
        } else {
            let g: u64 = ((top as u64) << 32) | (bot as u64);
            prism[1] = g.into();
        }
    }
}
