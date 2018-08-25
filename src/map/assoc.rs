// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn assoc(prism: Line, k: Unit, v: Unit) -> Unit {
    let guide: Guide = prism[1].into();
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let mut pop: Pop = prism[2 + root_gap as usize].into();

    let mut s = {
        let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
        if segment.is_aliased() {
            unalias_root(segment, pop, anchor_gap, root_gap, guide)
        } else { segment }
    };

    let hash = ValueUnit::from(k).hash();
    let hash_stack = hash;
    let chunks = MAX_LEVELS;
    let x = pop.child_idx(hash_stack & MASK);
    let y = pop.key_idx(hash_stack & MASK);
    if !pop.any_idx(hash_stack & MASK) {
        // place key in root
        // return
        unimplemented!()
    } else {
        // line to [pop, child] in unaliased segment
        // hash_stack and chunks
        unimplemented!()
    }
}

fn unalias_root(mut segment: Segment, pop: Pop, anchor_gap: u32, root_gap: u32, guide: Guide) -> Segment {
    let combined_count = pop.child_pop_count() + pop.key_pop_count();
    let (used_units, cap) = {
        let non_pop = anchor_gap + root_gap + 4 /*anchor, prism, guide, pop*/;
        (non_pop + 2 * combined_count,
         non_pop + 2 * combined_count.next_power_of_two())
    };
    let mut s = Segment::with_capacity(cap);
    for i in 1..used_units {
        s[i] = segment[i];
    }
    let child_base = anchor_gap + root_gap + 4 /*anchor, prism, guide, pop*/;
    for i in 0..pop.child_pop_count() {
        let idx = (i << 1) + 1;
        Segment::from(s[child_base + idx]).alias();
    }
    let key_base = child_base + 2 * pop.child_pop_count();
    for i in 0..(2 * pop.key_pop_count()) {
        ValueUnit::from(s[key_base + i]).split();
    }
    if guide.has_meta() {
        ValueUnit::from(s[3 + anchor_gap + guide.meta_gap()]).split();
    }
    if segment.unalias() == 0 {
        for i in 0..pop.child_pop_count() {
            let idx = (i << 1) + 1;
            Segment::from(s[child_base + idx]).unalias();
        }
        for i in 0..(2 * pop.key_pop_count()) {
            ValueUnit::from(s[key_base + i]).retire();
        }
        if guide.has_meta() {
            ValueUnit::from(s[3 + anchor_gap + guide.meta_gap()]).retire();
        }
    }
    s
}

