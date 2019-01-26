// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn chunk_at(hash: u32, chunk_idx: u32) -> u32 {
    (hash >> (chunk_idx * BITS)) & MASK
}

pub fn address(child_count: u32, key_count: u32, has_vals: u32) -> u32 {
    (child_count << 1) + (key_count << has_vals)
}

pub fn child_assoc(mut child_pop: AnchoredLine, k: Unit, hash: u32, has_vals: u32)
                   -> Result<AnchoredLine, AnchoredLine> {
    for chunk_idx in 1..MAX_LEVELS {
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
            child_pop = c.line_at(p.children_below(chunk) << 1);
        } else if p.has_key(chunk) {
            let child_count = p.child_count();
            let idx = p.keys_below(chunk);
            let key_idx = address(child_count, idx, has_vals);
            let k2 = c.get(key_idx).value_unit();
            if k.value_unit().eq(k2) {
                return Err(c.line_at(key_idx));
            } else {
                let (coll_pop, coll_child, key_slot) = collision_stalk(
                    chunk_idx + 1, hash, k2.hash(), c.line_at(key_idx), has_vals);
                let d = if has_vals == 1 { c } else {
                    let key_count = p.key_count();
                    let end = (child_count << 1) + key_count;
                    let d = if c.has_index(end) { c } else {
                        let s = Segment::new(size(end + 1));
                        c.at(0..end).to(s);
                        c.unalias();
                        Segment::free(c);
                        child_pop.set(1, s.unit());
                        s
                    };
                    d.at((key_idx + 1)..end).shift_up(1);
                    d
                };
                let c_idx = p.children_below(chunk) << 1;
                d.at(c_idx..key_idx).shift_up(2);
                d.set(c_idx, coll_pop);
                d.set(c_idx + 1, coll_child.unit());
                child_pop.set(0, p.flip_key(chunk).flip_child(chunk).into());
                return Ok(key_slot);
            }
        } else {
            return Ok(new_key_assoc(child_pop, p, c, chunk, has_vals));
        }
    }
    return chaining_assoc(child_pop, k, has_vals);
}

pub fn chaining_assoc(mut child_pop: AnchoredLine, k: Unit, has_vals: u32)
                      -> Result<AnchoredLine, AnchoredLine> {
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
        let k2 = c.get(i << has_vals).value_unit();
        if k.value_unit().eq(k2) {
            return Err(c.line_at(i << has_vals));
        }
    }
    let key_index = key_count << has_vals;
    let d = if c.has_index(key_index + has_vals) { c } else {
        let s = Segment::new(key_index + has_vals + 1);
        c.at(0..key_index).to(s);
        c.unalias();
        Segment::free(c);
        child_pop.set(1, s.unit());
        s
    };
    child_pop.set(0, Unit::from(key_count + 1));
    return Ok(d.line_at(key_index));
}

pub fn collision_stalk(skip_chunks: u32, hash: u32, hash2: u32, key2: AnchoredLine, has_vals: u32)
                       -> (Unit, Segment, AnchoredLine) {
    let leaf = Segment::new(size(2 << has_vals));
    let shared_chunks = common_chunks(hash, hash2);
    let (leaf_pop, k_idx) = if shared_chunks == MAX_LEVELS {
        (Unit::from(2u32), 0)
    } else {
        let c = chunk_at(hash, shared_chunks);
        let p = Pop::new().flip_key(c)
            .flip_key(chunk_at(hash2, shared_chunks));
        (p.unit(), p.keys_below(c))
    };
    {
        let k2_idx = (1 - k_idx) << has_vals;
        leaf.set(k2_idx, key2[0]);
        leaf.set(k2_idx + has_vals, key2[0 + has_vals as i32]);
    }
    let (mut u, mut s) = (leaf_pop, leaf);
    for chunk_idx in (skip_chunks..shared_chunks).rev() {
        let t = Segment::new(size(2));
        t.set(0, u);
        t.set(1, s.unit());
        s = t;
        u = Pop::new().flip_child(chunk_at(hash, chunk_idx)).unit();
    }
    (u, s, leaf.line_at(k_idx << has_vals))
}

pub fn new_key_assoc(child_pop: AnchoredLine, p: Pop, c: Segment, chunk: u32,
                     has_vals: u32) -> AnchoredLine {
    let child_count = p.child_count();
    let key_count = p.key_count();
    let new_address = address(child_count, p.key_count(), has_vals);
    let s = if c.has_index(new_address + has_vals) { c } else {
        let unit_count = address(child_count, key_count + 1, has_vals);
        let s = Segment::new(size(unit_count));
        c.at(0..(unit_count - (1 << has_vals))).to(s);
        c.unalias();
        Segment::free(c);
        child_pop.set(1, s.unit());
        s
    };
    let key_idx = p.keys_below(chunk);
    let idx = address(child_count, key_idx, has_vals);
    {
        let kvs_above = (key_count - key_idx) << has_vals;
        s.line_at(idx).span(kvs_above).shift_up(1 << has_vals);
    }
    child_pop.set(0, p.flip_key(chunk).into());
    s.line_at(idx)
}

pub fn unalias_child(p: Pop, has_vals: u32, c: Segment) -> Segment {
    let child_count = p.child_count();
    let key_count = p.key_count();
    let unit_count = address(child_count, key_count, has_vals);
    let s = Segment::new(size(unit_count));
    c.at(0..unit_count).to(s);
    for i in 0..child_count {
        c[1 + (i << 1)].segment().alias();
    }
    let kvs = c.line_at(child_count << 1).span(key_count << has_vals);
    kvs.split();
    if c.unalias() == 0 {
        for i in 0..child_count {
            c[1 + (i << 1)].segment().unalias();
        }
        kvs.retire();
        Segment::free(c);
    }
    s
}

pub fn assoc(prism: AnchoredLine, k: Unit, hash: u32, has_vals: u32)
             -> (Guide, Result<AnchoredLine, AnchoredLine>) {
    let guide = unaliased_root(Guide::hydrate(prism), has_vals);
    let p = Pop::from(guide.root[-1]);
    //println!("Count: {:2} {:?}", guide.count, p);
    let chunk = hash & MASK;
    if p.has_child(chunk) {
        let child_pop = guide.root.offset((p.children_below(chunk) << 1) as i32);
        (guide, child_assoc(child_pop, k, hash, has_vals))
    } else if p.has_key(chunk) {
        let child_count = p.child_count();
        let idx = p.keys_below(chunk);
        let root_idx = address(child_count, idx, has_vals);
        let k2 = guide.root.get(root_idx as i32).value_unit();
        if k.value_unit().eq(k2) {
            (guide, Err(guide.root.offset(root_idx as i32)))
        } else {
            let (coll_pop, coll_child, key_slot) = collision_stalk(
                1, hash, k2.hash(), guide.root.offset(root_idx as i32), has_vals);
            let g = if has_vals == 1 { guide } else {
                let key_count = p.key_count();
                let end = address(child_count, key_count, 0);
                let root_idx = address(child_count, idx, has_vals);
                let g = if guide.root.has_index(end as i32) { guide } else {
                    let cap = guide.root.index + size(end + 1);
                    let s = Segment::new(cap);
                    guide.segment().at(0..(guide.root.index + end)).to(s);
                    let mut g = guide;
                    g.prism = guide.prism.with_seg(s);
                    guide.segment().unalias();
                    Segment::free(guide.segment());
                    g.reroot()
                };
                let r = g.root.index();
                g.root.offset(root_idx as i32).span(key_count - idx).shift_up(1);
                g
            };
            let c_idx = (p.children_below(chunk) << 1) as i32;
            g.root.offset(c_idx).span(root_idx - c_idx as u32).shift_up(2);
            g.root.set(c_idx, coll_pop);
            g.root.set(c_idx + 1, coll_child.unit());
            g.root.set(-1, p.flip_key(chunk).flip_child(chunk).into());
            (g, Ok(key_slot))
        }
    } else {
        guide.root.set(-1, p.flip_key(chunk).into());
        let child_count = p.child_count();
        let key_count = p.key_count();
        let idx = p.keys_below(chunk);
        let root_idx = address(child_count, idx, has_vals);
        let root_end = address(child_count, key_count, has_vals);
        let root_above = (key_count - idx) << has_vals;
        if guide.root.has_index((root_end + has_vals) as i32) {
            guide.root.offset(root_idx as i32).span(root_above).shift_up(1 << has_vals);
            (guide, Ok(guide.root.offset(root_idx as i32)))
        } else {
            let root_units = address(child_count, key_count + 1, has_vals);
            let g = {
                let cap = guide.root.index + size(root_units);
                let s = Segment::new(cap);
                let mut g = guide;
                g.prism = guide.prism.with_seg(s);
                g.reroot()
            };
            guide.segment().at(0..(guide.root.index + root_idx)).to(g.segment());
            let keys_above = guide.root.offset(root_idx as i32).span(root_above);
            keys_above.to_offset(g.segment(), guide.root.index() + root_idx + (1 << has_vals));
            guide.segment().unalias();
            Segment::free(guide.segment());
            (g, Ok(g.root.offset(root_idx as i32)))
        }
    }
}

pub fn unalias_root(guide: Guide, has_vals: u32) -> Guide {
    let (child_count, key_count) = {
        let p = Pop::from(guide.root[-1]);
        (p.child_count(), p.key_count())
    };
    let root_units = address(child_count, key_count, has_vals);
    let g = {
        let cap = guide.root.index + size(root_units);
        let s = Segment::new(cap);
        let mut g = guide;
        g.prism = guide.prism.with_seg(s);
        g.reroot()
    };
    guide.segment().at(0..(guide.root.index + root_units)).to(g.segment());
    guide.split_meta();
    for i in 0..(child_count as i32) {
        guide.root[1 + (i << 1)].segment().alias();
    }
    let kvs = guide.root.offset((child_count << 1) as i32).span(key_count << has_vals);
    kvs.split();
    if guide.segment().unalias() == 0 {
        guide.retire_meta();
        for i in 0..(child_count as i32) {
            guide.root[1 + (i << 1)].segment().unalias();
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

