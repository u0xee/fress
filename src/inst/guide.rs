// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;

/// The Guide structure is hydrated from its in-memory representation, 64 bits in length.
/// The top 32 bits contain the hash, the bottom 32 bits contain the byte count.
/// The two highest order bits of the bottom 32 bits represent two booleans:
/// is there a hash, and is there meta data.

/// `Top 32 bits  [ Hash  (32) ]`
/// `Bottom bits  [ Fraction (32) ]`
/// `             [ Year-Month-Day (23) ]`
/// `             [ Hour-Min-Sec Offset-Hour-Min (29) ]`
///


#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub year: u32,
    pub month: u8,
    pub day: u8,

    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub nano: u32,

    pub off_neg: u8,
    pub off_hour: u8,
    pub off_min: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Guide {
    pub hash: u32,
    pub point: Point,
    pub prism: AnchoredLine,
}

pub fn field(width: u32) -> u32 {
    (1 << width) - 1
}

impl Guide {
    pub fn units() -> u32 { if cfg!(target_pointer_width = "32") { 4 } else { 2 } }

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
            Guide::hydrate_chunks(prism, prism[1].into(), prism[2].into(), prism[3].into(), prism[4].into())
        } else {
            let g: u64 = prism[1].into();
            let h: u64 = prism[2].into();
            Guide::hydrate_chunks(prism, (g >> 32) as u32, g as u32, (h >> 32) as u32, h as u32)
        }
    }

    pub fn hydrate_chunks(prism: AnchoredLine, hash: u32, nano: u32, date: u32, time: u32) -> Guide {
        let year    =  (date >>  9) & field(14);
        let month    = ((date >>  5) & field(4)) as u8;
        let day      = ( date        & field(5)) as u8;
        let hour     = ((time >> 24) & field(5)) as u8;
        let min      = ((time >> 18) & field(6)) as u8;
        let sec      = ((time >> 12) & field(6)) as u8;
        let off_neg  = ((time >> 11) & field(1)) as u8;
        let off_hour = ((time >>  6) & field(5)) as u8;
        let off_min  = ( time        & field(6)) as u8;

        Guide { hash, prism, point: Point {nano, year, month, day, hour, min, sec, off_neg, off_hour, off_min} }
    }


    pub fn store_at(&self, mut prism: AnchoredLine) {
        let p = &self.point;
        let date = p.year << 9 | (p.month as u32) << 5 | p.day as u32;
        let time = (p.hour as u32) << 24 | (p.min as u32) << 18 | (p.sec as u32) << 12 |
            (p.off_neg as u32) << 11 | (p.off_hour as u32) << 6 | p.off_min as u32;
        if cfg!(target_pointer_width = "32") {
            prism[1] = self.hash.into();
            prism[2] = p.nano.into();
            prism[3] = date.into();
            prism[4] = time.into();
        } else {
            let g: u64 = ((self.hash as u64) << 32) | (p.nano as u64);
            let h: u64 = ((date as u64) << 32) | (time as u64);
            prism[1] = g.into();
            prism[2] = h.into();
        }
    }

    pub fn store(self) -> Guide {
        self.store_at(self.prism);
        self
    }
}

