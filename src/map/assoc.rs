// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

// A | P G Meta Pop | Pop C Pop C | K V K V _ _ _ _
pub fn assoc(prism: AnchoredLine, has_vals: u32, k: Unit, hash: u32, v: Unit) -> (Unit, Unit) {
    let guide = unaliased_root(Guide::hydrate(prism), has_vals);
    let guide: Guide = prism[1].into();
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let pop: Pop = prism[2 + root_gap as usize].into();

    // is child present?
    // no. Then either placing key in root, or colliding key in root.
    //
    let hash = ValueUnit::from(k).hash();
    let hash_stack = hash;
    let chunks = MAX_LEVELS;
    let unset = un_set(guide);

    if !pop.contains(hash_stack & MASK) {
        // place key in root
        let child_count = pop.child_pop_count();
        let key_count = pop.key_pop_count();
        let used_units = anchor_gap + root_gap + 4 /*anchor, prism, guide, pop*/ +
            (child_count << 1) + (key_count << unset);
        let free_units = s.capacity() - used_units;
        let s = if free_units >= (1 << unset) {
            s
        } else {
            let total_count = child_count + key_count;
            let content_room = total_count.next_power_of_two() << 1;
            let cap = anchor_gap + root_gap + 4 /*anchor, prism, guide, pop*/ +
                content_room;
            let mut segment = Segment::with_capacity(cap);
            for i in 1..used_units {
                segment[i] = s[i];
            }
            Segment::free(s);
            segment
        };
        let keys_below_count = pop.keys_below(hash_stack & MASK);
        // A P G | P | P C P C | K V K V K V K V
        //                     | K K K K
        //

        // return
        unimplemented!()
    } else {
        // line to [pop, child] in unaliased segment
        // hash_stack and chunks

        unimplemented!()
    }
}

pub fn unalias_root(guide: Guide, has_vals: u32) -> Guide {
    let (child_count, key_count) = {
        let pop = Pop::from(guide.root[0]);
        (pop.child_pop_count(), pop.key_pop_count())
    };
    let root_units = (child_count << 1) + (key_count << has_vals);
    let g = {
        let cap = guide.root.index + 1 /*pop*/ + size(root_units);
        let s = Segment::new(cap);
        let mut g = guide;
        g.prism = guide.prism.with_seg(s);
        g.reroot()
    };
    guide.segment().at(0..(guide.root.index + root_units)).to(g.segment());
    guide.split_meta();
    for i in 0..child_count {
        guide.root[2 + (i << 1)].segment().alias();
    }
    let kvs = guide.root.offset(1 + (child_count << 1)).span(key_count << has_vals);
    kvs.split();
    if guide.segment().unalias() == 0 {
        guide.retire_meta();
        for i in 0..child_count {
            guide.root[2 + (i << 1)].segment().unalias();
        }
        kvs.retire();
        Segment::free(guide.segment());
    }
    g
}

pub fn unaliased_root(guide: Guide, has_vals: u32) -> Guide {
    if guide.segment().is_aliased() {
        unalias_root(guide, has_vals)
    } else {
        guide
    }
}

