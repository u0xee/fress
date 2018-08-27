// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn pop(prism: Line) -> (Unit, Unit) {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    if count <= TAIL_CAP {
        pop_untailed(prism, guide, count)
    } else {
        pop_tailed(prism, guide, count)
    }
}

fn pop_untailed(prism: Line, guide: Guide, count: u32) -> (Unit, Unit) {
    if count == 0 {
        (prism.into(), Unit::from(0))
    } else {
        // TODO shrink tail slot if shrinking down (copy root contents down by one)
        // maybe...need to look at expansion code in conj

        let anchor_gap = guide.prism_to_anchor_gap();
        let root_gap = guide.guide_to_root_gap();
        let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
        let mut s = if segment.is_aliased() {
            unalias_root(segment, anchor_gap, root_gap, count, guide)
        } else { segment };

        s[2 + anchor_gap] = guide.dec().into();
        let first_root = 3 + anchor_gap + root_gap;
        (Unit::from(s), s[first_root + count - 1])
    }
}

fn pop_tailed(prism: Line, guide: Guide, count: u32) -> (Unit, Unit) {
    let tailoff = (count - 1) & !MASK;
    let tail_count = count - tailoff;
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, anchor_gap, root_gap, count, guide)
    } else { segment };
    if tail_count > 1 {
        let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
        let mut tail: Segment = s[first_root - 1].into();
        let mut t = if tail.is_aliased() {
            let mut t = Segment::new(TAIL_CAP);
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
        s[2 + anchor_gap] = guide.dec().into();
        (Unit::from(s), t[first_root + tail_count - 1])
    } else {
        pop_tailed_drained(prism, guide, count, tailoff, tail_count, s)
    }
}

fn pop_tailed_drained(prism: Line, guide: Guide, count: u32, tailoff: u32, tail_count: u32,
                      mut s: Segment) -> (Unit, Unit) {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    let popped: Unit = {
        let mut tail: Segment = s[first_root - 1].into();
        let p = tail[1];
        if tail.is_aliased() {
            ValueUnit::from(p).split();
            if tail.unalias() == 0 {
                ValueUnit::from(p).retire();
                Segment::free(tail);
            }
        } else {
            Segment::free(tail);
        }
        p
    };
    s[2 + anchor_gap] = guide.dec().into();

    let tail_path = tailoff - TAIL_CAP;
    if tail_path == 0 {
        (s.into(), popped)
    } else {
        let last_index = tail_path - 1;
        let path_diff = tail_path ^ last_index;
        use std::cmp::Ordering;
        let ret = match digit_count(last_index).cmp(&digit_count(path_diff)) {
            Ordering::Less    => { shrink_height(prism, guide, count, tailoff, tail_count, last_index, s) },
            Ordering::Equal   => { shrink_root(prism, guide, count, tailoff, tail_count, last_index, s) },
            Ordering::Greater => { shrink_child(prism, guide, count, tailoff, tail_count, last_index, s) },
        };
        (ret, popped)
    }
}

fn unlink_tail(mut s: Segment, height: u32) -> Segment {
    for i in 0..height {
        if !s.is_aliased() {
            let mut t: Segment = s[1u32].into();
            Segment::free(s);
            s = t;
        } else {
            return unlink_tail_aliased(s, height - i);
        }
    }
    s
}

fn unlink_tail_aliased(mut s: Segment, height: u32) -> Segment {
    let mut tail = {
        let mut t = s;
        for i in 0..height {
            t = t[1u32].into();
        }
        t
    };
    tail.alias();
    for i in 0..(height + 1) {
        if s.unalias() == 0 {
            let mut t: Segment = s[1u32].into();
            Segment::free(s);
            s = t;
        } else {
            break
        }
    }
    tail
}

fn shrink_height(prism: Line, guide: Guide, count: u32, tailoff: u32,
                 tail_count: u32, last_index: u32, mut header: Segment) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    let mut tail_path_head: Segment = header[first_root + 1].into();
    let path_height = trailing_zero_digit_count(last_index >> BITS);
    let mut tail = unlink_tail(tail_path_head, path_height);
    header[first_root - 1] = tail.into();
    let cap = anchor_gap + root_gap + 3 /*anchor, prism, guide*/ + TAIL_CAP;
    let mut s = if header.capacity() >= cap {
        header
    } else {
        let mut s = Segment::with_capacity(cap);
        for i in 1..cap {
            s[i] = header[i];
        }
        Segment::free(header);
        s
    };
    let mut content: Segment = s[first_root].into();
    for i in 0..TAIL_CAP {
        s[first_root + i] = content[1 + i];
    }
    if content.is_aliased() {
        for i in 0..TAIL_CAP {
            ValueUnit::from(content[1 + i]).split();
        }
        if content.unalias() == 0 {
            for i in 0..TAIL_CAP {
                ValueUnit::from(content[1 + i]).retire();
            }
            Segment::free(content);
        }
    } else {
        Segment::free(content);
    }
    s.into()
}

fn shrink_root(prism: Line, guide: Guide, count: u32, tailoff: u32,
               tail_count: u32, last_index: u32, mut header: Segment) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    let root_count = root_content_count(tailoff);
    let mut tail_path_head: Segment = header[first_root + root_count - 1].into();
    let path_height = trailing_zero_digit_count(last_index >> BITS);
    let mut tail = unlink_tail(tail_path_head, path_height);
    header[first_root - 1] = tail.into();
    header.into()
}

fn shrink_child(prism: Line, guide: Guide, count: u32, tailoff: u32,
                tail_count: u32, last_index: u32, mut header: Segment) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    let root_count = root_content_count(tailoff);

    let zero_count = trailing_zero_digit_count(last_index);
    let digit_count = digit_count(last_index);
    let (mut stack, root_idx) = {
        let rev = reverse_digits(last_index >> (zero_count * BITS),
                                 digit_count - zero_count);
        (rev >> BITS, rev & MASK)
    };
    let stack_digit_count = digit_count - zero_count - 1;
    let mut tail_path_head: Segment = {
        let mut child = Line::from(header).offset((first_root + root_idx) as isize);

        for _ in 0..stack_digit_count {
            let mut s = Segment::from(child[0]);
            let digit = stack & MASK;
            stack = stack >> BITS;

            if !s.is_aliased() {
                child = Line::from(s).offset((1 + digit) as isize);
            } else {
                let mut c = Segment::new(digit + 1);
                for i in 1..(digit + 1 + 1) {
                    c[i] = s[i];
                }
                for i in 1..(digit + 1 + 1) {
                    Segment::from(c[i]).alias();
                }
                if s.unalias() == 0 {
                    for i in 1..(digit + 1 + 1) {
                        Segment::from(c[i]).unalias();
                    }
                }
                let next_child = Line::from(c).offset((1 + digit) as isize);
                child[0] = c.into();
                child = next_child;
            }
        }
        child[0].into()
    };
    let mut tail = unlink_tail(tail_path_head, zero_count - 1);
    header[first_root - 1] = tail.into();
    header.into()
}

