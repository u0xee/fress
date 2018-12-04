// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use super::conj::*;

pub fn pop(prism: AnchoredLine) -> (Unit, Unit) {
    let guide = unaliased_root(Guide::hydrate(prism));
    if guide.count <= TAIL_CAP {
        pop_untailed(guide)
    } else {
        pop_tailed(guide)
    }
}

pub fn pop_untailed(guide: Guide) -> (Unit, Unit) {
    if guide.count == 0 {
        (guide.segment().unit(), Unit::from(0))
    } else {
        let popped = guide.root[(guide.count - 1) as i32];
        (guide.dec_count().store().segment().unit(), popped)
    }
}

pub fn pop_tailed(guide: Guide) -> (Unit, Unit) {
    let tail_count = tail_count(guide.count);
    if tail_count != 1 {
        let tail = {
            let tail = guide.root[-1].segment();
            if tail.is_aliased() {
                let s = Segment::new(TAIL_CAP);
                let tails = tail.at(0..tail_count);
                tails.to(s);
                tails.split();
                if tail.unalias() == 0 {
                    tails.retire();
                    Segment::free(tail);
                }
                guide.root.set(-1, s.unit());
                s
            } else {
                tail
            }
        };
        let popped = tail[tail_count - 1];
        (guide.dec_count().store().segment().unit(), popped)
    } else {
        pop_tailed_drained(guide)
    }
}

pub fn pop_tailed_drained(guide: Guide) -> (Unit, Unit) {
    let tail = guide.root[-1].segment();
    let popped = tail[0];
    if tail.is_aliased() {
        popped.value_unit().split();
        if tail.unalias() == 0 {
            popped.value_unit().retire();
            Segment::free(tail);
        }
    } else {
        Segment::free(tail);
    }
    let tailoff = tailoff(guide.count);
    let tail_path = tailoff - TAIL_CAP;
    if tail_path == 0 {
        (guide.dec_count().store().segment().unit(), popped)
    } else {
        let last_index = tail_path - 1;
        let path_diff = tail_path ^ last_index;
        use std::cmp::Ordering;
        let ret = match digit_count(last_index).cmp(&digit_count(path_diff)) {
            Ordering::Less    => { shrink_height(guide, last_index) },
            Ordering::Equal   => { shrink_root(guide, tailoff, last_index) },
            Ordering::Greater => { shrink_child(guide, tailoff, last_index) },
        };
        (ret, popped)
    }
}

pub fn unlink_tail(mut s: Segment, height: u32) -> Segment {
    for i in 0..height {
        if !s.is_aliased() {
            let t = s[0].segment();
            Segment::free(s);
            s = t;
        } else {
            return unlink_tail_aliased(s, height - i);
        }
    }
    s
}

pub fn unlink_tail_aliased(mut s: Segment, height: u32) -> Segment {
    let tail = {
        let mut t = s;
        for _ in 0..height {
            t = t[0].segment();
        }
        t
    };
    tail.alias();
    for _ in 0..(height + 1) {
        if s.unalias() == 0 {
            let t = s[0].segment();
            Segment::free(s);
            s = t;
        } else {
            break
        }
    }
    tail
}

pub fn shrink_height(guide: Guide, last_index: u32) -> Unit {
    let tail_path_head = guide.root[1].segment();
    let path_height = trailing_zero_digit_count(last_index >> BITS);
    let tail = unlink_tail(tail_path_head, path_height);
    guide.root.set(-1, tail.unit());
    let g = if guide.root.has_index(TAIL_CAP as i32 - 1) {
        guide
    } else {
        let s = Segment::new(guide.root.index + TAIL_CAP);
        guide.segment().at(0..(guide.root.index + 1)).to(s);
        let mut g = guide;
        g.prism = guide.prism.with_seg(s);
        Segment::free(guide.segment());
        g.reroot()
    };
    let c = g.root[0].segment();
    c.at(0..TAIL_CAP).to_offset(g.segment(), g.root.index);
    if c.is_aliased() {
        let contents = c.at(0..TAIL_CAP);
        contents.split();
        if c.unalias() == 0 {
            contents.retire();
            Segment::free(c);
        }
    } else {
        Segment::free(c);
    }
    g.dec_count().store().segment().unit()
}

pub fn shrink_root(guide: Guide, tailoff: u32, last_index: u32) -> Unit {
    let root_count = root_content_count(tailoff);
    let tail_path_head = guide.root[(root_count - 1) as i32].segment();
    let path_height = trailing_zero_digit_count(last_index >> BITS);
    let tail = unlink_tail(tail_path_head, path_height);
    guide.root.set(-1, tail.unit());
    guide.dec_count().store().segment().unit()
}

pub fn shrink_child(guide: Guide, tailoff: u32, last_index: u32) -> Unit {
    let zero_count = trailing_zero_digit_count(last_index >> BITS);
    let digit_count = digit_count(last_index);
    let c = create_path(guide.root, last_index, digit_count, digit_count - zero_count);
    if zero_count == 0 {
        println!("{:?}", guide);
    }
    let tail = unlink_tail(c[0].segment(), zero_count);
    guide.root.set(-1, tail.unit());
    guide.dec_count().store().segment().unit()
}
