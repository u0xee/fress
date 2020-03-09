// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;

/// The Guide structure is hydrated from its in-memory representation, 64 bits in length.
/// The top 32 bits contain the hash, the bottom 32 bits contain the collection's count.
/// The top uses one bit to indicate presence of a hash, and 31 bits to store the hash.
/// So a collection's hash will always have a zero as the highest order bit.
/// The two highest order bits of the bottom 32 bits also represent two booleans:
/// is the representation compact (no info unit), and is there meta data.
/// So a collection's count resides in the 30 lowest order bits.

/// `Top 32 bits  [                     Hash  (32) ]`
/// `Bottom bits  [ Compact? | Meta? |  Count (30) ]`
///

#[derive(Copy, Clone, Debug)]
pub struct Guide {
    pub hash: u32,

    pub is_compact_bit: u32,
    pub has_meta_bit: u32,
    pub count: u32,

    pub prism: AnchoredLine,
    pub root: AnchoredLine,
}

impl Guide {
    pub fn units() -> u32 {
        if cfg!(target_pointer_width = "32") { 2 } else { 1 }
    }

    pub fn segment(&self) -> Segment {
        self.prism.segment()
    }

    pub fn set_hash(mut self, hash: u32) -> Guide {
        self.hash = hash;
        self
    }

    pub fn clear_hash(mut self) -> Guide {
        self.hash = 0;
        self
    }

    pub fn has_hash(&self) -> bool {
        self.hash != 0
    }

    pub fn set_meta(mut self) -> Guide {
        self.has_meta_bit = 1;
        self
    }

    pub fn clear_meta(mut self) -> Guide {
        self.has_meta_bit = 0;
        self
    }

    pub fn has_meta(&self) -> bool {
        self.has_meta_bit == 1
    }

    pub fn meta_line(&self) -> AnchoredLine {
        self.prism.offset(1 + Guide::units() as i32)
    }

    pub fn split_meta(&self) {
        if self.has_meta() {
            self.meta_line()[0].handle().split();
        }
    }

    pub fn retire_meta(&self) {
        if self.has_meta() {
            self.meta_line()[0].handle().retire();
        }
    }

    pub fn clear_compact(mut self) -> Guide {
        self.is_compact_bit = 0;
        self
    }

    pub fn inc_count(mut self) -> Guide {
        self.count = self.count + 1;
        self.clear_hash()
    }

    pub fn dec_count(mut self) -> Guide {
        self.count = self.count - 1;
        self.clear_hash()
    }

    pub fn reroot(mut self) -> Guide {
        let root_offset = 1 /*prism*/ + Guide::units() + self.has_meta_bit
            + (!self.is_compact_bit & 1);
        self.root = self.prism.offset(root_offset as i32);
        self
    }

    pub fn hydrate(prism: AnchoredLine) -> Guide {
        if cfg!(target_pointer_width = "32") {
            Guide::hydrate_top_bot(prism, prism[1].into(), prism[2].into())
        } else {
            let g: u64 = prism[1].into();
            Guide::hydrate_top_bot(prism, (g >> 32) as u32, g as u32)
        }
    }

    pub fn hydrate_top_bot(prism: AnchoredLine, top: u32, bot: u32) -> Guide {
        let hash = top;
        let is_compact_bit = (bot >> 31) & 1;
        let has_meta_bit   = (bot >> 30) & 1;
        let count =  {
            let low_30 = (1 << 30) - 1;
            bot & low_30
        };

        let root_offset = 1 /*prism*/ + Guide::units() + has_meta_bit + (!is_compact_bit & 1);
        let root = prism.offset(root_offset as i32);

        Guide { hash, has_meta_bit, count, is_compact_bit, prism, root }
    }

    pub fn store_at(&self, mut prism: AnchoredLine) {
        let top: u32 = self.hash;
        let bot: u32 = (self.is_compact_bit << 31) | (self.has_meta_bit << 30) | self.count;
        if cfg!(target_pointer_width = "32") {
            prism[1] = top.into();
            prism[2] = bot.into();
        } else {
            let g: u64 = ((top as u64) << 32) | (bot as u64);
            prism[1] = g.into();
        }
    }

    pub fn store(self) -> Guide {
        self.store_at(self.prism);
        self
    }

    pub fn store_hash(self) -> Guide {
        let prism = self.prism;
        let top: u32 = self.hash;
        let bot: u32 = (self.is_compact_bit << 31) | (self.has_meta_bit << 30) | self.count;
        if cfg!(target_pointer_width = "32") {
            prism.store_hash(1, top.into());
            prism.store_hash(2, bot.into());
        } else {
            let g: u64 = ((top as u64) << 32) | (bot as u64);
            prism.store_hash(1, g.into())
        }
        self
    }
}

