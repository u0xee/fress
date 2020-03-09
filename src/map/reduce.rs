// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use vector::tear_down::{NodeRecord, NodeRecordStack, BLANK};
use transduce::{inges, inges_kv, last_call, Process};

pub fn ingest_keys(first_key: AnchoredLine, key_count: u32, process_stack: &mut [Box<dyn Process>],
                   has_vals: u32) -> Option<Value> {
    for i in 0..key_count {
        let key = first_key.offset((i << has_vals) as i32);
        let x = key.line().star() as *const Value;
        let y = unsafe { &* x };
        if let Some(ret) = if has_vals == 0 { inges(process_stack, y) } else {
            let w = key.offset(1).line().star() as *const Value;
            let z = unsafe { &* w };
            inges_kv(process_stack, y, z)
        } {
            return Some(ret);
        }
    }
    None
}

pub fn reduce(prism: AnchoredLine, process_stack: &mut [Box<dyn Process>], has_vals: u32) -> Value {
    let guide = Guide::hydrate(prism);
    let (child_count, key_count) = {
        let p = Pop::from(guide.root[-1]);
        (p.child_count(), p.key_count())
    };
    let first_key = guide.root.offset((child_count << 1) as i32);
    if let Some(ret) = ingest_keys(first_key, key_count, process_stack, has_vals) {
        return ret;
    }
    let stack_space = [BLANK; 8];
    let mut stack = NodeRecordStack::new(stack_space.as_ptr());
    stack.push(NodeRecord {
        first_child: guide.root,
        child_count,
        height: 0,
        on_boundary: false,
        current_child: None,
    });
    while !stack.is_empty() {
        if let Some(ret) = step(&mut stack, process_stack, has_vals) {
            return ret;
        }
    }
    last_call(process_stack)
}

pub fn step(stack: &mut NodeRecordStack, process_stack: &mut [Box<dyn Process>],
            has_vals: u32) -> Option<Value> {
    let top = stack.top();
    if top.height == MAX_LEVELS - 1 {
        if let Some(ret) = collision_children(top, process_stack, has_vals) {
            return Some(ret);
        }
        stack.pop();
        return None;
    }
    let next = match top.current_child {
        Some(i) => { i + 1 },
        None    => { 0 },
    };
    let cap = top.child_count;
    if next < cap {
        let idx = (next << 1) as i32;
        let (child_count, key_count) = {
            let p = Pop::from(top.first_child[idx]);
            (p.child_count(), p.key_count())
        };
        let c = top.first_child[idx + 1].segment();
        let first_key = c.line_at(child_count << 1);
        if let Some(ret) = ingest_keys(first_key, key_count, process_stack, has_vals) {
            return Some(ret);
        }
        top.current_child = Some(next);
        if child_count != 0 {
            stack.push(NodeRecord {
                first_child: c.line_at(0),
                child_count,
                height: top.height + 1,
                on_boundary: false,
                current_child: None,
            });
        }
    } else {
        stack.pop();
    }
    None
}

pub fn collision_children(node: &NodeRecord, process_stack: &mut [Box<dyn Process>],
                          has_vals: u32) -> Option<Value> {
    for i in 0..node.child_count {
        let idx = (i << 1) as i32;
        let key_count = node.first_child[idx].u32();
        let c = node.first_child[idx + 1].segment();
        let first_key = c.line_at(0);
        if let Some(ret) = ingest_keys(first_key, key_count, process_stack, has_vals) {
            return Some(ret);
        }
    }
    None
}

