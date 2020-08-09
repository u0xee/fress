// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn assoc(prism: AnchoredLine, idx: u32, x: Unit) -> (Unit, Unit) {
    let guide = Guide::hydrate(unaliased(prism));
    match idx.cmp(&guide.count) {
        Ordering::Less => {
            if guide.count <= TAIL_CAP {
                assoc_untailed(guide, idx, x)
            } else {
                assoc_tailed(guide, idx, x)
            }
        },
        Ordering::Equal   => { (super::conj::conj(prism, x), Handle::nil().unit()) },
        Ordering::Greater => { panic!("Index out of bounds: {} in vector of count {}", idx, guide.count); }
    }
}

pub fn assoc_untailed(guide: Guide, idx: u32, x: Unit) -> (Unit, Unit) {
    let popped = guide.root[idx as i32];
    guide.root.set(idx as i32, x);
    (guide.clear_hash().store().segment().unit(), popped)
}

pub fn assoc_tailed(guide: Guide, idx: u32, x: Unit) -> (Unit, Unit) {
    let tailoff = tailoff(guide.count);
    if idx >= tailoff {
        let tail_count = tail_count(guide.count);
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
        let tail_idx = idx - tailoff;
        let popped = tail[tail_idx];
        tail.set(tail_idx, x);
        (guide.clear_hash().store().segment().unit(), popped)
    } else {
        assoc_tailed_tree(guide, idx, x, tailoff)
    }
}

pub fn create_path_width(root: AnchoredLine, path: u32, path_widths: u32, height: u32) -> AnchoredLine {
    let mut shift = height * BITS;
    let mut curr = {
        shift -= BITS;
        root.offset(last_digit(path >> shift) as i32)
    };
    for _ in 0..(height - 1) {
        let s = curr[0].segment();
        let (digit, last_sibling_idx) = {
            shift -= BITS;
            (last_digit(path >> shift), last_digit(path_widths >> shift))
        };
        if !s.is_aliased() {
            curr = s.line_at(digit);
        } else {
            let t = {
                let t = Segment::new(size(last_sibling_idx + 1));
                let range = s.at(0..(last_sibling_idx + 1));
                range.to(t);
                range.split();
                if s.unalias() == 0 {
                    range.retire();
                    Segment::free(s);
                }
                t
            };
            curr.set(0, t.unit());
            curr = t.line_at(digit);
        }
    }
    assert_eq!(shift, 0);
    curr
}

pub fn assoc_tailed_tree(guide: Guide, idx: u32, x: Unit, tailoff: u32) -> (Unit, Unit) {
    let last_index = tailoff - 1;
    let digit_count = digit_count(last_index);
    let path_widths = path_widths(tailoff, idx);
    let c = create_path_width(guide.root, idx, path_widths, digit_count);
    let popped = c[0];
    c.set(0, x);
    (guide.clear_hash().store().segment().unit(), popped)
}
