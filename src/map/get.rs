// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use super::assoc::{address, chunk_at};

pub fn get_child(mut child_pop: AnchoredLine, k: Unit, hash: u32, has_vals: u32)
                 -> Option<AnchoredLine> {
    for chunk_idx in 1..MAX_LEVELS {
        let p = Pop::from(child_pop[0]);
        let c = child_pop[1].segment();
        let chunk = chunk_at(hash, chunk_idx);
        if p.has_child(chunk) {
            child_pop = c.line_at(p.children_below(chunk) << 1);
        } else if p.has_key(chunk) {
            let child_count = p.child_count();
            let idx = p.keys_below(chunk);
            let key_idx = address(child_count, idx, has_vals);
            let k2 = c.get(key_idx);
            return if k.value_unit().eq(k2.value_unit()) {
                Some(c.line_at(key_idx))
            } else {
                None
            };
        } else {
            return None;
        }
    }
    let key_count = child_pop[0].u32();
    let c = child_pop[1].segment();
    for i in 0..key_count {
        let k2 = c.get(i << has_vals);
        if k.value_unit().eq(k2.value_unit()) {
            return Some(c.line_at(i << has_vals));
        }
    }
    None
}

pub fn get(prism: AnchoredLine, k: Unit, hash: u32, has_vals: u32) -> Option<AnchoredLine> {
    let guide = Guide::hydrate(prism);
    let p = Pop::from(guide.root[-1]);
    let chunk = hash & MASK;
    if p.has_child(chunk) {
        let idx = p.children_below(chunk) << 1;
        get_child(guide.root.offset(idx as i32), k, hash, has_vals)
    } else if p.has_key(chunk) {
        let child_count = p.child_count();
        let idx = p.keys_below(chunk);
        let root_idx = address(child_count, idx, has_vals);
        let k2 = guide.root.get(root_idx as i32);
        if k.value_unit().eq(k2.value_unit()) {
            Some(guide.root.offset(root_idx as i32))
        } else {
            None
        }
    } else {
        None
    }
}

