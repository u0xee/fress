// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
use dispatch::*;
use super::guide::Guide;
use super::*;

pub fn conj(prism: Line, x: Unit) -> Unit {
    let guide: Guide = prism[1].into();
    let count = guide.count();
    if count <= TAIL_CAP {
        conj_untailed(prism, x, guide, count)
    } else {
        conj_tailed(prism, x, guide, count)
    }
}

fn conj_untailed(prism: Line, x: Unit, guide: Guide, count: u32) -> Unit {
    if count == TAIL_CAP {
        conj_untailed_complete(prism, x, guide, count)
    } else {
        conj_untailed_incomplete(prism, x, guide, count)
    }
}

fn conj_untailed_complete(prism: Line, x: Unit, guide: Guide, count: u32) -> Unit {
    let mut tail = Segment::new(TAIL_CAP);
    tail[1] = x;
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();

    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, anchor_gap, root_gap, count, guide)
    } else { segment };
    let first_root_element = 3 + anchor_gap + root_gap;
    s[first_root_element - 1] = tail.into();
    s[2 + anchor_gap] = guide.inc().into();
    Unit::from(s)
}

fn unalias_root(mut segment: Segment, anchor_gap: u32, root_gap: u32, root_count: u32, guide: Guide) -> Segment {
    let used_units = anchor_gap + root_gap + root_count + 3 /*anchor, prism, guide*/;
    // TODO
    // breathing room?
    let mut s = Segment::with_capacity(used_units);
    for i in 1..used_units {
        s[i] = segment[i];
    }
    for i in (used_units - root_count)..used_units {
        ValueUnit::from(s[i]).split()
    }
    if guide.has_meta() {
        // TODO
        // + 1 if external hash
        ValueUnit::from(s[3 + anchor_gap]).split()
    }
    if segment.unalias() == 0 {
        for i in (used_units - root_count)..used_units {
            ValueUnit::from(s[i]).retire()
        }
        if guide.has_meta() {
            // TODO
            // same as above
            ValueUnit::from(s[3 + anchor_gap]).retire()
        }
        Segment::free(segment)
    }
    s
}

fn conj_untailed_incomplete(prism: Line, x: Unit, guide: Guide, count: u32) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    if segment.is_aliased() {
        conj_untailed_incomplete_aliased(prism, x, guide, count, anchor_gap, segment)
    } else {
        conj_untailed_incomplete_unaliased(prism, x, guide, count, anchor_gap, segment)
    }
}

fn conj_untailed_incomplete_aliased(prism: Line, x: Unit, guide: Guide, count: u32,
                                    anchor_gap: u32, mut segment: Segment) -> Unit {
    let root_gap = guide.guide_to_root_gap();
    let used_units = anchor_gap + root_gap + count + 3 /*anchor, prism, guide*/;

    let new_count = (count + 1).next_power_of_two();
    let new_cap = used_units + (new_count - count);
    let (shift, guide) = if new_count == TAIL_CAP
        { (1, guide.inc_guide_to_root_gap()) } else { (0, guide) };
    let mut s = Segment::with_capacity(new_cap + shift);

    for i in 1..(used_units - count) {
        s[i] = segment[i]
    }
    for i in (used_units - count)..used_units {
        s[i + shift] = segment[i]
    }

    for i in (used_units - count)..used_units {
        ValueUnit::from(s[i + shift]).split()
    }
    if guide.has_meta() {
        ValueUnit::from(s[3 + anchor_gap]).split()
    }

    if segment.unalias() == 0 {
        for i in (used_units - count)..used_units {
            ValueUnit::from(s[i + shift]).retire()
        }
        if guide.has_meta() {
            ValueUnit::from(s[3 + anchor_gap]).retire()
        }
        Segment::free(segment)
    }

    s[used_units + shift] = x;
    s[2 + anchor_gap] = guide.inc().into();
    Unit::from(s)
}

fn conj_untailed_incomplete_unaliased(prism: Line, x: Unit, guide: Guide, count: u32,
                                      anchor_gap: u32, mut segment: Segment) -> Unit {
    let root_gap = guide.guide_to_root_gap();
    let used_units = anchor_gap + root_gap + count + 3 /*anchor, prism, guide*/;
    let cap = segment.capacity();

    if used_units == cap {
        let new_count = (count + 1).next_power_of_two();
        let new_cap = used_units + (new_count - count);
        let (shift, guide) = if new_count == TAIL_CAP
            { (1, guide.inc_guide_to_root_gap()) } else { (0, guide) };
        let mut s = Segment::with_capacity(new_cap + shift);

        for i in 1..(used_units - count) {
            s[i] = segment[i]
        }
        for i in (used_units - count)..used_units {
            s[i + shift] = segment[i]
        }

        s[used_units + shift] = x;
        s[2 + anchor_gap] = guide.inc().into();
        Unit::from(s)
    } else {
        segment[used_units] = x;
        segment[2 + anchor_gap] = guide.inc().into();
        Unit::from(segment)
    }
}

fn conj_tailed(prism: Line, x: Unit, guide: Guide, count: u32) -> Unit {
    let tailoff = (count - 1) & !MASK;
    let tail_count = count - tailoff;
    if tail_count != TAIL_CAP {
        let anchor_gap = guide.prism_to_anchor_gap();
        let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
        let mut s = if segment.is_aliased() {
            unalias_root(segment, guide.prism_to_anchor_gap(),
                         guide.guide_to_root_gap(), root_content_count(tailoff), guide)
        } else { segment };
        let first_root_element = 3 + anchor_gap + guide.guide_to_root_gap();
        let mut tail = Segment::from(s[first_root_element - 1]);
        let mut t = if tail.is_aliased() {
            let mut t = Segment::new(TAIL_CAP);
            for i in 1..(tail_count + 1) {
                t[i] = tail[i];
            }
            for i in 1..(tail_count + 1) {
                ValueUnit::from(t[i]).split()
            }
            if tail.unalias() == 0 {
                for i in 1..(tail_count + 1) {
                    ValueUnit::from(t[i]).retire()
                }
                Segment::free(tail)
            }
            t
        } else { tail };
        t[tail_count + 1] = x;
        s[first_root_element - 1] = t.into();
        s[2 + anchor_gap] = guide.inc().into();
        Unit::from(s)
    } else {
        conj_tailed_complete(prism, x, guide, count, tailoff, tail_count)
    }
}

fn conj_tailed_complete(prism: Line, x: Unit, guide: Guide, count: u32,
                        tailoff: u32, tail_count: u32) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, guide.prism_to_anchor_gap(),
                     guide.guide_to_root_gap(), root_content_count(tailoff), guide)
    } else { segment };
    let last_index = tailoff - 1;
    let path_diff = tailoff ^ last_index;
    use std::cmp::Ordering;
    match digit_count(last_index).cmp(&digit_count(path_diff)) {
        Ordering::Less    => { growing_height(prism, x, guide, count, tailoff, tail_count, s) },
        Ordering::Equal   => { growing_root(prism, x, guide, count, tailoff, tail_count, s) },
        Ordering::Greater => { growing_child(prism, x, guide, count, tailoff, tail_count, s) },
    }
}

fn growing_height(prism: Line, x: Unit, guide: Guide, count: u32,
                  tailoff: u32, tail_count: u32, mut header: Segment) -> Unit {
    let mut child = Segment::new(ARITY);
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    for i in 0..ARITY {
        child[i + 1] = header[i + first_root];
    }
    header[first_root] = child.into();
    header[first_root + 1] = header[first_root - 1];
    let path_height = trailing_zero_digit_count(tailoff >> BITS);
    for _ in 0..path_height {
        let mut c = Segment::new(4);
        c[1] = header[first_root + 1];
        header[first_root + 1] = c.into();
    }
    let mut t = Segment::new(TAIL_CAP);
    t[1] = x;
    header[first_root - 1] = t.into();
    header[2 + anchor_gap] = guide.inc().into();

    let mut h = Segment::with_capacity(first_root + 4);
    for i in 1..(first_root + 2) {
        h[i] = header[i];
    }
    Segment::free(header);
    Unit::from(h)
}

fn growing_root(prism: Line, x: Unit, guide: Guide, count: u32,
                tailoff: u32, tail_count: u32, mut header: Segment) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let root_count = root_content_count(tailoff);

    let used_units = anchor_gap + root_gap + root_count + 3 /*anchor, prism, guide*/;
    let cap = header.capacity();

    let mut h = if used_units == cap {
        let new_count = (root_count + 1).next_power_of_two();
        let new_cap = used_units + (new_count - root_count);
        let mut s = Segment::with_capacity(new_cap);

        for i in 1..used_units {
            s[i] = header[i];
        }
        Segment::free(header);
        s
    } else { header };
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    h[used_units] = h[first_root - 1];
    let path_height = trailing_zero_digit_count(tailoff >> BITS);
    for i in 0..path_height {
        let mut c = Segment::new(4);
        c[1] = h[used_units];
        h[used_units] = c.into();
    }
    let mut t = Segment::new(TAIL_CAP);
    t[1] = x;
    h[first_root - 1] = t.into();
    h[2 + anchor_gap] = guide.inc().into();
    Unit::from(h)
}

fn growing_child(prism: Line, x: Unit, guide: Guide, count: u32,
                 tailoff: u32, tail_count: u32, mut header: Segment) -> Unit {
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let root_count = root_content_count(tailoff);

    let used_units = anchor_gap + root_gap + root_count + 3 /*anchor, prism, guide*/;
    let cap = header.capacity();


    let zero_count = trailing_zero_digit_count(tailoff);
    let ultimate_idx = (tailoff >> (zero_count * BITS)) & MASK;
    let digit_count = digit_count(tailoff);
    let (mut stack, root_idx) = {
        let rev = reverse_digits(tailoff >> ((zero_count + 1) * BITS),
                                 digit_count - zero_count - 1);
        (rev >> BITS, rev & MASK)
    };
    let stack_digit_count = digit_count - zero_count - 2;
    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;

    let mut child = {
        let mut child = Line::from(header).offset((first_root + root_idx) as isize);

        for _ in 0..stack_digit_count {
            let mut s = Segment::from(child[0]);
            let digit = stack & MASK;
            stack = stack >> BITS;

            if !s.is_aliased() {
                child = Line::from(s).offset((1 + digit) as isize);
            } else {
                let unit_count = (digit + 1).next_power_of_two();
                let mut c = Segment::new(unit_count);
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
        child
    };

    let mut s = Segment::from(child[0]);
    let mut leaf_space: Line = if s.is_aliased() {
        let unit_count = (ultimate_idx + 1).next_power_of_two();
        let mut t = Segment::new(unit_count);
        for i in 1..(ultimate_idx + 1) {
            t[i] = s[i];
        }
        for i in 1..(ultimate_idx + 1) {
            Segment::from(t[i]).alias();
        }
        if s.unalias() == 0 {
            for i in 1..(ultimate_idx + 1) {
                Segment::from(t[i]).unalias();
            }
            Segment::free(s);
        }
        child[0] = t.into();
        Line::from(t).offset((1 + ultimate_idx) as isize)
    } else {
        let cap = s.capacity();
        if cap == 1 + ultimate_idx {
            let unit_count = (ultimate_idx + 1).next_power_of_two();
            let mut t = Segment::new(unit_count);
            for i in 1..(ultimate_idx + 1) {
                t[i] = s[i];
            }
            Segment::free(s);
            child[0] = t.into();
            Line::from(t).offset((1 + ultimate_idx) as isize)
        } else {
            Line::from(s).offset((1 + ultimate_idx) as isize)
        }
    };

    let first_root = anchor_gap + root_gap + 3 /*anchor, prism, guide*/;
    leaf_space[0] = header[first_root - 1];
    for _ in 0..(zero_count - 1) {
        let mut c = Segment::new(4);
        c[1] = leaf_space[0];
        leaf_space[0] = c.into();
    }
    let mut t = Segment::new(TAIL_CAP);
    t[1] = x;
    header[first_root - 1] = t.into();
    header[2 + anchor_gap] = guide.inc().into();
    Unit::from(header)
}

fn reverse_digits(mut x: u32, digit_count: u32) -> u32 {
    let mut ret = 0u32;
    for i in 0..digit_count {
        ret = (ret << BITS) | (x & MASK);
        x = x >> BITS;
    }
    ret
}

fn significant_bits(x: u32) -> u32 {
    /*bits in a u32*/ 32 - x.leading_zeros()
}

fn digit_count(x: u32) -> u32 {
    (significant_bits(x) + BITS - 1) / BITS
}

fn trailing_zero_digit_count(x: u32) -> u32 {
    x.trailing_zeros() / BITS
}

fn root_content_count(tailoff: u32) -> u32 {
    let last_index = tailoff - 1;
    let dc = digit_count(last_index);
    let last_root_index = last_index >> (BITS * (dc - 1));
    last_root_index + 1
}

fn digit(x: u32, idx: u8) -> u8 {
    (x >> (idx as u32 * BITS)) as u8
}

fn digit_iter(x: u32, digits: u8) {
    // Digit iterator struct
}
