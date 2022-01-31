// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

// Push a new item into the tree.
// Splits out the case of a small vector (hasn't grown a tail yet)
pub fn conj(prism: AnchoredLine, x: Unit) -> Unit {
    let guide = Guide::hydrate(unaliased(prism));
    if guide.count <= TAIL_CAP {
        conj_untailed(guide, x)
    } else {
        conj_tailed(guide, x)
    }
}

// For an untailed vector, all items live in one chunk of memory.
// It may be time to grow a tail, or add to the existing array
// which may need resizing if full.
pub fn conj_untailed(guide: Guide, x: Unit) -> Unit {
    if guide.count == TAIL_CAP { // complete, grow a tail
        let tail = {
            let tail = Segment::new(TAIL_CAP);
            tail.set(0, x);
            tail
        };
        guide.root.set(-1, tail.unit());
        guide.inc_count().store().segment().unit()
    } else { // incomplete
        if guide.root.has_index(guide.count as i32) { // has room
            guide.root.set(guide.count as i32, x);
            guide.inc_count().store().segment().unit()
        } else { // resize (doubling the array)
            let width = size(guide.count);
            let grew_tail_bit = guide.is_compact_bit & is_arity_bit(width);
            let g = {
                let s = Segment::new(guide.root.index + (width | grew_tail_bit));
                let mut g = guide;
                g.prism = guide.prism.with_seg(s);
                g.is_compact_bit = g.is_compact_bit & !grew_tail_bit;
                g.reroot()
            };
            guide.segment().at(0..guide.root.index).to(g.segment());
            guide.root.span(guide.count).to_offset(g.segment(), g.root.index);
            guide.segment().unalias();
            Segment::free(guide.segment());
            g.root.set(g.count as i32, x);
            g.inc_count().store().segment().unit()
        }
    }
}

// General case of a vector with a tail.
// Commonly, just append to tail, otherwise call tailed_complete
pub fn conj_tailed(guide: Guide, x: Unit) -> Unit {
    let tail_count = tail_count(guide.count);
    if tail_count != TAIL_CAP {
        let tail = guide.root[-1].segment();
        if tail.is_aliased() {
            let t = Segment::new(TAIL_CAP);
            let tails = tail.at(0..tail_count);
            tails.to(t);
            tails.split();
            if tail.unalias() == 0 {
                tails.retire();
                Segment::free(tail);
            }
            t.set(tail_count, x);
            guide.root.set(-1, t.unit());
            guide.inc_count().store().segment().unit()
        } else {
            tail.set(tail_count, x);
            guide.inc_count().store().segment().unit()
        }
    } else {
        conj_tailed_complete(guide, x)
    }
}

// Using logic similar to an odometer rolling over,
// decide if the tree will grow in height, expand the root,
// or (commonly) expand a child segment below the root.
pub fn conj_tailed_complete(guide: Guide, x: Unit) -> Unit {
    let tailoff = guide.count - TAIL_CAP;
    let last_index = tailoff - 1;
    let path_diff = tailoff ^ last_index;
    match digit_count(last_index).cmp(&digit_count(path_diff)) {
        Ordering::Less    => { growing_height(guide, x, tailoff) },
        Ordering::Equal   => { growing_root(guide, x, tailoff) },
        Ordering::Greater => { growing_child(guide, x, tailoff) },
    }
}

pub fn path_of_height(height: u32, mut end: Unit) -> Unit {
    for _ in 0..height {
        let c = Segment::new(size(1) /*4*/);
        c.set(0, end);
        end = c.unit();
    }
    end
}

// When the tree grows in height, the root contents
// are moved out into their own array below the root
pub fn growing_height(guide: Guide, x: Unit, tailoff: u32) -> Unit {
    let g = {
        let s = {
            let s = Segment::new(guide.root.index + size(3) /*4*/);
            guide.segment().at(0..guide.root.index).to(s);
            let child = {
                let c = Segment::new(ARITY);
                guide.root.span(ARITY).to_offset(c, 0);
                c
            };
            s.set(guide.root.index, child.unit());
            s
        };
        let mut g = guide;
        g.prism = guide.prism.with_seg(s);
        guide.segment().unalias();
        Segment::free(guide.segment());
        g.reroot()
    };
    let path = path_of_height(trailing_zero_digit_count(tailoff >> BITS), g.root[-1]);
    g.root.set(1, path);
    let tail = {
        let t = Segment::new(TAIL_CAP);
        t.set(0, x);
        t
    };
    g.root.set(-1, tail.unit());
    g.inc_count().store().segment().unit()
}

// A new path is appended to the root contents, and the full
// tail is stored at the end of the path.
pub fn growing_root(guide: Guide, x: Unit, tailoff: u32) -> Unit {
    let root_count = root_content_count(tailoff);
    let g = if guide.root.has_index(root_count as i32) {
        guide
    } else {
        let g = {
            let grown_root_count = root_count + 1;
            let width = size(grown_root_count + 1 /*tail*/);
            let cap = guide.root.index - 1 /*tail*/ + (width | is_arity_bit(width));
            let s = Segment::new(cap);
            let mut g = guide;
            g.prism = guide.prism.with_seg(s);
            g.reroot()
        };
        guide.segment().at(0..(guide.root.index + root_count)).to(g.segment());
        guide.segment().unalias();
        Segment::free(guide.segment());
        g
    };
    let path = path_of_height(trailing_zero_digit_count(tailoff >> BITS), g.root[-1]);
    g.root.set(root_count as i32, path);
    let tail = {
        let t = Segment::new(TAIL_CAP);
        t.set(0, x);
        t
    };
    g.root.set(-1, tail.unit());
    g.inc_count().store().segment().unit()
}

#[derive(Copy, Clone)]
pub struct Digits {
    pub path: u32,
    pub shift: u32,
    pub count: i32,
}

use std::fmt;
impl fmt::Debug for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Digits {{ path: {:X}, shift: {:X}, count: {:X} }}", self.path, self.shift, self.count)
    }
}

impl Digits {
    pub fn new(path: u32, height: u32, count: u32) -> Self {
        Digits { path, count: count as i32, shift: height * BITS }
    }

    pub fn pop(&mut self) -> u32 {
        self.shift -= BITS;
        self.count -= 1;
        last_digit(self.path >> self.shift)
    }
}

pub fn unalias_edge_path(mut curr: AnchoredLine, d: &mut Digits) -> AnchoredLine {
    let count = d.count as u32;
    for _ in 0..count {
        let s = curr[0].segment();
        let digit = d.pop();
        if !s.is_aliased() {
            curr = s.line_at(digit);
        } else {
            let t = {
                let t = Segment::new(size(digit + 1));
                let range = s.at(0..(digit + 1));
                range.to(t);
                range.alias();
                if s.unalias() == 0 {
                    range.unalias();
                    Segment::free(s);
                }
                t
            };
            curr.set(0, t.unit());
            curr = t.line_at(digit);
        }
    }
    curr
}

pub fn unalias_grown_index(curr: AnchoredLine, grown_digit: u32) -> AnchoredLine {
    let s = curr[0].segment();
    if !s.is_aliased() {
        if s.has_index(grown_digit) {
            s.line_at(grown_digit)
        } else {
            let t = Segment::new(size(grown_digit + 1));
            s.at(0..grown_digit).to(t);
            s.unalias();
            Segment::free(s);
            curr.set(0, t.unit());
            t.line_at(grown_digit)
        }
    } else {
        let t = Segment::new(size(grown_digit + 1));
        let range = s.at(0..grown_digit);
        range.to(t);
        range.alias();
        if s.unalias() == 0 {
            //panic!("Race condition unalias_grown_index");
            range.unalias();
            Segment::free(s);
        }
        curr.set(0, t.unit());
        t.line_at(grown_digit)
    }
}

// An existing chiled below the root will have the full tail
// appended to its contents
pub fn growing_child(guide: Guide, x: Unit, tailoff: u32) -> Unit {
    let zero_count = trailing_zero_digit_count(tailoff >> BITS);
    let digit_count = digit_count(tailoff);
    let c = {
        let mut d = Digits::new(tailoff, digit_count, digit_count - zero_count - 2);
        let c = unalias_edge_path(guide.root.offset(d.pop() as i32), &mut d);
        unalias_grown_index(c, d.pop())
    };
    let path = path_of_height(zero_count, guide.root[-1]);
    c.set(0, path);
    let tail = {
        let t = Segment::new(TAIL_CAP);
        t.set(0, x);
        t
    };
    guide.root.set(-1, tail.unit());
    guide.inc_count().store().segment().unit()
}

