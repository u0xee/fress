// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn nth(prism: Line, idx: u32) -> Unit {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    if idx >= count {
        panic!("Index out of bounds: {} in vector of count {}", idx, count);
    }
    if count <= TAIL_CAP {
        nth_untailed(prism, idx, guide, count)
    } else {
        nth_tailed(prism, idx, guide, count)
    }
}

fn nth_untailed(prism: Line, idx: u32, guide: Guide, count: u32) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let first_root = 3 + anchor_gap + root_gap;
    segment[first_root + idx]
}

fn nth_tailed(prism: Line, idx: u32, guide: Guide, count: u32) -> Unit {
    let tailoff = (count - 1) & !MASK;
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let first_root = 3 + anchor_gap + root_gap;
    if idx >= tailoff {
        let tail = Segment::from(segment[first_root - 1]);
        tail[1 + (idx - tailoff)]
    } else {
        let digit_count = digit_count(tailoff - 1);
        let (mut stack, root_idx) = {
            let rev = reverse_digits(idx, digit_count);
            (rev >> BITS, rev & MASK)
        };
        let mut x = segment[first_root + root_idx];
        for _ in 0..(digit_count - 1) {
            let digit = stack & MASK;
            stack = stack >> BITS;
            x = Line::from(x)[(1 + digit) as usize]
        }
        x
    }
}


