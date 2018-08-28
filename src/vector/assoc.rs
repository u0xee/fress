// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn assoc(prism: Line, idx: u32, x: Unit) -> (Unit, Unit) {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    use std::cmp::Ordering;
    match idx.cmp(&count) {
        Ordering::Less => {
            if count <= TAIL_CAP {
                assoc_untailed(prism, idx, x, guide, count)
            } else {
                assoc_tailed(prism, idx, x, guide, count)
            }
        },
        Ordering::Equal   => { (super::conj::conj(prism, x), Value::NIL) },
        Ordering::Greater => { panic!("Index out of bounds: {} in vector of count {}", idx, count); }
    }
}

fn assoc_untailed(prism: Line, idx: u32, x: Unit, guide: Guide, count: u32) -> (Unit, Unit) {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, anchor_gap, root_gap, count, guide)
    } else { segment };
    let first_root = 3 + anchor_gap + root_gap;
    let popped = s[first_root + idx];
    s[first_root + idx] = x;
    (Unit::from(s), popped)
}

fn assoc_tailed(prism: Line, idx: u32, x: Unit, guide: Guide, count: u32) -> (Unit, Unit) {
    let tailoff = (count - 1) & !MASK;
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, anchor_gap, root_gap, count, guide)
    } else { segment };
    let first_root = 3 + anchor_gap + root_gap;
    if idx >= tailoff {
        let tail_count = count - tailoff;
        let mut tail: Segment = s[first_root - 1].into();
        let mut t = if tail.is_aliased() {
            let mut t = Segment::with_capacity(tail.capacity());
            for i in 1..(tail_count + 1) {
                t[i] = tail[i];
            }
            for i in 1..(tail_count + 1) {
                ValueUnit::from(t[i]).split();
            }
            if tail.unalias() == 0 {
                for i in 1..(tail_count + 1) {
                    ValueUnit::from(t[i]).retire();
                }
                Segment::free(tail)
            }
            s[first_root - 1] = t.into();
            t
        } else { tail };
        let tail_idx = idx - tailoff;
        let popped = t[1 + tail_idx];
        t[1 + tail_idx] = x;
        (Unit::from(s), popped)
    } else {
        assoc_tailed_tree(prism, idx, x, guide, count, anchor_gap, root_gap, tailoff, s)
    }
}

fn assoc_tailed_tree(prism: Line, idx: u32, x: Unit, guide: Guide, count: u32,
                     anchor_gap: u32, root_gap: u32, tailoff: u32, mut header: Segment) -> (Unit, Unit) {
    let first_root = 3 + anchor_gap + root_gap;
    let digit_count = digit_count(tailoff - 1);
    let (mut stack, root_idx) = {
        let rev = reverse_digits(idx, digit_count);
        (rev >> BITS, rev & MASK)
    };
    let mut child = Line::from(header).offset((first_root + root_idx) as isize);
    let mut path_width_stack = path_width_stack(tailoff, idx) >> BITS;

    for _ in 0..(digit_count - 1) {
        let mut s = Segment::from(child[0]);
        let digit = stack & MASK;
        stack = stack >> BITS;
        let last_sibling_idx = path_width_stack & MASK;
        path_width_stack = path_width_stack >> BITS;

        if !s.is_aliased() {
            child = Line::from(s).offset((1 + digit) as isize);
        } else {
            let mut c = Segment::with_capacity(s.capacity());
            for i in 0..(last_sibling_idx + 1) {
                c[1 + i] = s[1 + i];
            }
            for i in 0..(last_sibling_idx + 1) {
                ValueUnit::from(c[1 + i]).split();
            }
            if s.unalias() == 0 {
                for i in 0..(last_sibling_idx + 1) {
                    ValueUnit::from(c[1 + i]).retire();
                }
            }
            let next_child = Line::from(c).offset((1 + digit) as isize);
            child[0] = c.into();
            child = next_child;
        }
    }

    let popped = child[0];
    child[0] = x;
    (Unit::from(header), popped)
}


