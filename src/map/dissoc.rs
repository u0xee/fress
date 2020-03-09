// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use super::assoc::{unaliased_root, unalias_child, address, chunk_at};

pub fn child_dissoc(guide: Guide, root_child_pop: AnchoredLine, k: Unit, hash: u32, has_vals: u32) -> Guide {
    let mut child_stack = [root_child_pop; 8];
    for chunk_idx in 1..MAX_LEVELS {
        let child_pop = child_stack[chunk_idx as usize - 1];
        let p = Pop::from(child_pop[0]);
        let c = {
            let c = child_pop[1].segment();
            if !c.is_aliased() { c } else {
                let s = unalias_child(p, has_vals, c);
                child_pop.set(1, s.unit());
                s
            }
        };
        let chunk = chunk_at(hash, chunk_idx);
        if p.has_child(chunk) {
            child_stack[chunk_idx as usize] = c.line_at(p.children_below(chunk) << 1);
        } else if p.has_key(chunk) {
            let idx = p.keys_below(chunk);
            let key_idx = address(p.child_count(), idx, has_vals);
            let k2 = c.get(key_idx).handle();
            if k.handle().eq(k2) {
                k2.retire();
                if has_vals == 1 {
                    c.get(key_idx + 1).handle().retire();
                }
                {
                    let next_key = key_idx + 1 + has_vals;
                    let keys_above = p.key_count() - idx - 1;
                    let above = c.line_at(next_key).span(keys_above << has_vals);
                    above.shift_down(1 << has_vals);
                }
                let pop = p.flip_key(chunk);
                child_pop.set(0, pop.into());
                if pop.key_count() == 1 && pop.child_count() == 0 {
                    let mut child_idx = chunk_idx - 1;
                    while child_idx > 0 {
                        let pop = {
                            let p = child_stack[child_idx as usize - 1];
                            let c = child_stack[child_idx as usize];
                            let h = chunk_at(hash, child_idx);
                            merge_child(p, c, h, has_vals)
                        };
                        if pop.key_count() == 1 && pop.child_count() == 0 {
                            child_idx -= 1;
                        } else {
                            return guide.dec_count().store();
                        }
                    }
                }
                return guide.dec_count().store();
            } else {
                return guide; // not found
            }
        } else {
            return guide; // not found
        }
    }
    let child_pop = child_stack[MAX_LEVELS as usize - 1];
    let g = chaining_dissoc(guide, child_pop, k, has_vals);
    let remaining = child_pop[0].u32();
    if remaining == 1 {
        let mut child_idx = MAX_LEVELS - 1;
        while child_idx > 0 {
            let pop = {
                let p = child_stack[child_idx as usize - 1];
                let c = child_stack[child_idx as usize];
                let h = chunk_at(hash, child_idx);
                merge_child(p, c, h, has_vals)
            };
            if pop.key_count() == 1 && pop.child_count() == 0 {
                child_idx -= 1;
            } else {
                return g;
            }
        }
    }
    g
}

pub fn chaining_dissoc(guide: Guide, child_pop: AnchoredLine, k: Unit, has_vals: u32) -> Guide {
    let key_count = child_pop[0].u32();
    let c = {
        let c = child_pop[1].segment();
        if !c.is_aliased() { c } else {
            let s = Segment::new((key_count + 1) << has_vals);
            let kvs = c.at(0..(key_count << has_vals));
            kvs.split();
            if c.unalias() == 0 {
                kvs.retire();
                Segment::free(c);
            }
            child_pop.set(1, s.unit());
            s
        }
    };
    for i in 0..key_count {
        let idx = i << has_vals;
        let k2 = c.get(idx).handle();
        if k.handle().eq(k2) {
            k2.retire();
            if has_vals == 1 {
                c.get(idx + 1).handle().retire();
            }
            c.at((idx + 1 + has_vals)..(key_count << has_vals)).shift_down(1 << has_vals);
            child_pop.set(0, Unit::from(key_count - 1));
            return guide.dec_count().store();
        }
    }
    guide
}

// parent_pop -> ...[P C]...
//                     |
// child_pop  ->       [...[P C]...K...]
//                            |
//                            [K]
pub fn merge_child(parent_pop: AnchoredLine, child_pop: AnchoredLine,
                   hash_chunk: u32, has_vals: u32) -> Pop {
    let (k, v) = {
        let c = child_pop[1].segment();
        let ret = (c[0], c[has_vals]);
        c.unalias();
        Segment::free(c);
        ret
    };
    let p = Pop::from(parent_pop[0]);
    let c = parent_pop[1].segment();
    assert!(p.has_child(hash_chunk));
    let after = (p.children_below(hash_chunk) + 1) << 1;
    let pos = address(p.child_count(), p.keys_below(hash_chunk), has_vals);
    c.at(after..pos).shift_down(2);
    c.set(pos - 2, k);
    if has_vals == 1 {
        c.set(pos - 1, v);
    } else {
        let end = address(p.child_count(), p.key_count(), has_vals);
        c.at(pos..end).shift_down(1);
    }
    let new_p = p.flip_child(hash_chunk).flip_key(hash_chunk);
    parent_pop.set(0, new_p.into());
    new_p
}

pub fn dissoc(prism: AnchoredLine, k: Unit, hash: u32, has_vals: u32) -> Guide {
    let guide = unaliased_root(Guide::hydrate(prism), has_vals);
    let p = Pop::from(guide.root[-1]);
    let chunk = hash & MASK;
    if p.has_child(chunk) {
        let child_pop = guide.root.offset((p.children_below(chunk) << 1) as i32);
        let g = child_dissoc(guide, child_pop, k, hash, has_vals);
        let remaining = Pop::from(child_pop[0]);
        if remaining.key_count() == 1 && remaining.child_count() == 0 {
            let (k, v) = {
                let c = child_pop[1].segment();
                let ret = (c[0], c[has_vals]);
                c.unalias();
                Segment::free(c);
                ret
            };
            let after = (p.children_below(chunk) + 1) << 1;
            let pos = address(p.child_count(), p.keys_below(chunk), has_vals);
            g.root.offset(after as i32).span(pos - after).shift_down(2);
            g.root.set(pos as i32 - 2, k);
            if has_vals == 1 {
                g.root.set(pos as i32 - 1, v);
            } else {
                let end = address(p.child_count(), p.key_count(), has_vals);
                g.root.offset(pos as i32).span(end - pos).shift_down(1);
            }
            let new_p = p.flip_child(chunk).flip_key(chunk);
            g.root.set(-1, new_p.into());
        }
        g
    } else if p.has_key(chunk) {
        let child_count = p.child_count();
        let idx = p.keys_below(chunk);
        let root_idx = address(child_count, idx, has_vals);
        let k2 = guide.root.get(root_idx as i32).handle();
        if k.handle().eq(k2) {
            k2.retire();
            if has_vals == 1 {
                guide.root.get(root_idx as i32 + 1).handle().retire();
            }
            let next_key = root_idx + 1 + has_vals;
            let keys_above = p.key_count() - idx - 1;
            let above = guide.root.offset(next_key as i32).span(keys_above << has_vals);
            above.shift_down(1 << has_vals);
            guide.root.set(-1, p.flip_key(chunk).into());
            guide.dec_count().store()
        } else {
            guide
        }
    } else {
        guide
    }
}

