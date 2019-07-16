// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use vector::eq::{NodeRecord, NodeRecordStack, BLANK, eq_range};

pub fn eq(guide: Guide, o_guide: Guide, has_vals: u32) -> bool {
    if guide.count != o_guide.count { return false }
    if guide.has_hash() && o_guide.has_hash() && guide.hash != o_guide.hash { return false }
    let (child_count, key_count) = {
        let (pa, pb) = (guide.root[-1], o_guide.root[-1]);
        if pa != pb { return false }
        let p = Pop::from(pa);
        (p.child_count(), p.key_count())
    };
    let first_key = (child_count << 1) as i32;
    let (ka, kb) = (guide.root.offset(first_key), o_guide.root.offset(first_key));
    if !eq_range(ka, kb, key_count << has_vals) {
        return false;
    }
    if child_count == 0 { return true }
    let stack_space = [BLANK; 8];
    let mut stack = NodeRecordStack::new(stack_space.as_ptr());
    stack.push(NodeRecord {
        first_child: (guide.root, o_guide.root),
        child_count,
        height: 0,
        on_boundary: false,
        current_child: None,
    });
    while !stack.is_empty() {
        if !step(&mut stack, has_vals) {
            return false
        }
    }
    true
}

pub fn step(stack: &mut NodeRecordStack, has_vals: u32) -> bool {
    let top = stack.top();
    if top.height == MAX_LEVELS - 1 {
        if !eq_collision_children(top, has_vals) {
            return false;
        }
        stack.pop();
        return true;
    }
    let search_from = match top.current_child {
        None    => { 0 },
        Some(i) => { i + 1 },
    };
    let mut i = search_from;
    let cap = top.child_count;
    while i < cap {
        let idx = (i << 1) as i32;
        let (ca, cb) = (top.first_child.0[idx + 1], top.first_child.1[idx + 1]);
        if ca != cb {
            let (child_count, key_count) = {
                let (pa, pb) = (top.first_child.0[idx], top.first_child.1[idx]);
                if pa != pb { return false }
                let p = Pop::from(pa);
                (p.child_count(), p.key_count())
            };
            let (sa, sb) = (ca.segment(), cb.segment());
            if !eq_range(sa.line_at(child_count << 1), sb.line_at(child_count << 1),
                         key_count << has_vals) {
                return false;
            }
            if child_count != 0 {
                top.current_child = Some(i);
                stack.push(NodeRecord {
                    first_child: (sa.line_at(0), sb.line_at(0)),
                    child_count,
                    height: top.height + 1,
                    on_boundary: false,
                    current_child: None,
                });
                return true;
            }
        }
        i = i + 1;
    }
    stack.pop();
    return true;
}

pub fn eq_collision_children(node: &NodeRecord, has_vals: u32) -> bool {
    for i in 0..node.child_count {
        let idx = (i << 1) as i32;
        let (ca, cb) = (node.first_child.0[idx + 1], node.first_child.1[idx + 1]);
        if ca != cb {
            let key_count = {
                let (a, b) = (node.first_child.0[idx].u32(), node.first_child.1[idx].u32());
                if a != b { return false }
                a
            };
            let (sa, sb) = (ca.segment(), cb.segment());
            if !eq_collision_segments(sa, sb, key_count, has_vals) {
                return false;
            }
        }
    }
    true
}

pub fn eq_collision_segments(sa: Segment, sb: Segment, key_count: u32, has_vals: u32) -> bool {
    for i in 0..key_count {
        let idx = i << has_vals;
        let k = sa.get(idx).handle();
        match find_index_of_key(sb, k, key_count, has_vals) {
            None => { return false },
            Some(b_idx) => {
                let (v, v2) = (sa.get(idx + 1).handle(), sb.get(b_idx + 1).handle());
                if !v.eq(v2) {
                    return false
                }
            },
        }
    }
    true
}

pub fn find_index_of_key(s: Segment, k: Handle, key_count: u32, has_vals: u32) -> Option<u32> {
    for i in 0..key_count {
        let idx = i << has_vals;
        if k.eq(s.get(idx).handle()) {
            return Some(idx)
        }
    }
    None
}

