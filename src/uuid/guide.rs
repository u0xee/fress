// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;

#[derive(Copy, Clone, Debug)]
pub struct Guide {
    pub hash: u32,
    pub top: u64,
    pub bot: u64,

    pub prism: AnchoredLine,
}

impl Guide {
    pub fn units() -> u32 { if cfg!(target_pointer_width = "32") { 5 } else { 3 } }

    pub fn segment(&self) -> Segment { self.prism.segment() }

    pub fn set_hash(mut self, hash: u32) -> Guide {
        self.hash = hash;
        self
    }

    pub fn clear_hash(mut self) -> Guide {
        self.hash = 0;
        self
    }

    pub fn has_hash(&self) -> bool { self.hash != 0 }

    pub fn hydrate(prism: AnchoredLine) -> Guide {
        if cfg!(target_pointer_width = "32") {
            let hash: u32 =  prism[1].into();
            let top:  u64 = (prism[2].u64() << 32) | prism[3].u64();
            let bot:  u64 = (prism[4].u64() << 32) | prism[5].u64();
            Guide { hash, top, bot, prism }
        } else {
            let hash: u32 = prism[1].into();
            let top:  u64 = prism[2].into();
            let bot:  u64 = prism[3].into();
            Guide { hash, top, bot, prism }
        }
    }

    pub fn new(prism: AnchoredLine) -> Guide {
        Guide { hash: 0, top: 0, bot: 0, prism }
    }

    pub fn store_at(&self, mut prism: AnchoredLine) {
        if cfg!(target_pointer_width = "32") {
            prism[1] =   self.hash.into();
            prism[2] = ((self.top >> 32) as u32).into();
            prism[3] =  (self.top        as u32).into();
            prism[4] = ((self.bot >> 32) as u32).into();
            prism[5] =  (self.bot        as u32).into();
        } else {
            prism[1] = self.hash.into();
            prism[2] = self.top.into();
            prism[3] = self.bot.into();
        }
    }

    pub fn store(self) -> Guide {
        self.store_at(self.prism);
        self
    }

    pub fn store_hash(self) -> Guide {
        let prism = self.prism;
        if cfg!(target_pointer_width = "32") {
            prism.store_hash(1, self.hash.into());
        } else {
            prism.store_hash(1, self.hash.into());
        }
        self
    }
}

