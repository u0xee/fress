// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use super::tear_down::{NodeRecord, NodeRecordStack, BLANK};
use transducer::{ingest, last_call, Process};

pub struct Iter {
    pub x: usize,
}

pub fn iter(prism: AnchoredLine, process_stack: &mut [Box<Process>]) {

    unimplemented!()
}

pub fn reduce(prism: AnchoredLine, process_stack: &mut [Box<Process>]) -> Value {
    let guide = Guide::hydrate(prism);
    let count = guide.count;
    if count <= TAIL_CAP {
        for i in 0..count {
            let x = guide.root.offset(i as i32).line().star() as *const Value;
            let y = unsafe { &* x };
            if let Some(ret) = ingest(process_stack, y) {
                return ret;
            }
        }
        last_call(process_stack)
    } else {
        let tailoff = tailoff(guide.count);
        if tailoff == TAIL_CAP {
            for i in 0..TAIL_CAP {
                let x = guide.root.offset(i as i32).line().star() as *const Value;
                let y = unsafe { &* x };
                if let Some(ret) = ingest(process_stack, y) {
                    return ret;
                }
            }
            let tail = guide.root[-1].segment();
            for i in 0..(count - tailoff) {
                let x = tail.line_at(i).line().star() as *const Value;
                let y = unsafe { &* x };
                if let Some(ret) = ingest(process_stack, y) {
                    return ret;
                }
            }
            last_call(process_stack)
        } else {
            reduce_tree(guide, tailoff, process_stack)
        }
    }
}

pub fn reduce_tree(guide: Guide, tailoff: u32, process_stack: &mut [Box<Process>]) -> Value {
    let last_tree_index = tailoff - 1;
    let stack_space = [BLANK; 8];
    let mut stack = NodeRecordStack::new(stack_space.as_ptr());
    stack.push(NodeRecord {
        first_child: guide.root,
        child_count: root_content_count(tailoff),
        height: digit_count(last_tree_index),
        on_boundary: true,
        current_child: None,
    });
    while !stack.is_empty() {
        if let Some(ret) = step(&mut stack, process_stack, last_tree_index) {
            return ret;
        }
    }
    let tail = guide.root[-1].segment();
    for i in 0..(guide.count - tailoff) {
        let x = tail.line_at(i).line().star() as *const Value;
        let y = unsafe { &* x };
        if let Some(ret) = ingest(process_stack, y) {
            return ret;
        }
    }
    last_call(process_stack)
}

pub fn base_case(node: &NodeRecord, process_stack: &mut [Box<Process>]) -> Option<Value> {
    for i in 0..node.child_count {
        let a_tail = node.first_child[i as i32].segment();
        for j in 0..TAIL_CAP {
            let x = a_tail.line_at(j).line().star() as *const Value;
            let y = unsafe { &* x };
            if let Some(ret) = ingest(process_stack, y) {
                return Some(ret);
            }
        }
    }
    None
}

pub fn step(stack: &mut NodeRecordStack, process_stack: &mut [Box<Process>],
            last_tree_index: u32) -> Option<Value> {
    let top = stack.top();
    if top.height == 2 {
        if let Some(ret) = base_case(top, process_stack) {
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
        let idx = next;
        let s = top.first_child[idx as i32].segment();
        let r = child_record(top, last_tree_index, s, idx == cap - 1);
        top.current_child = Some(next);
        stack.push(r);
    } else {
        stack.pop();
    }
    None
}

pub fn child_record(top: &mut NodeRecord, last_tree_index: u32, s: Segment, last_child: bool)
                    -> NodeRecord {
    let mut r = NodeRecord {
        first_child: s.line_at(0),
        child_count: TAIL_CAP,
        height: top.height - 1,
        on_boundary: false,
        current_child: None,
    };
    if top.on_boundary && last_child {
        let child_idx = (last_tree_index >> (BITS * (r.height - 1))) & MASK;
        r.child_count = child_idx + 1;
        r.on_boundary = true;
    }
    r
}


