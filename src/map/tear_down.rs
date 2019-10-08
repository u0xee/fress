// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use vector::tear_down::{NodeRecord, NodeRecordStack, BLANK};

pub fn tear_down(prism: AnchoredLine, has_vals: u32) {
    // segment has 0 aliases
    let guide = Guide::hydrate(prism);
    guide.retire_meta();
    let (child_count, key_count) = {
        let p = Pop::from(guide.root[-1]);
        (p.child_count(), p.key_count())
    };
    guide.root.offset((child_count << 1) as i32).span(key_count << has_vals).retire();
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
        step(&mut stack, has_vals);
    }
    Segment::free(guide.segment());
}

pub fn step(stack: &mut NodeRecordStack, has_vals: u32) {
    let top = stack.top();
    if top.height == MAX_LEVELS - 1 {
        collision_children(top, has_vals);
        stack.pop();
        return;
    }
    let search_from = match top.current_child {
        Some(i) => {
            let idx = (i << 1) as i32;
            let s = top.first_child[idx + 1].segment();
            Segment::free(s);
            i + 1
        },
        None => { 0 },
    };
    let mut i = search_from;
    let cap = top.child_count;
    while i < cap {
        let idx = (i << 1) as i32;
        let c = top.first_child[idx + 1].segment();
        if c.unalias() == 0 {
            let (child_count, key_count) = {
                let p = Pop::from(top.first_child[idx]);
                (p.child_count(), p.key_count())
            };
            c.line_at(child_count << 1).span(key_count << has_vals).retire();
            if child_count == 0 {
                Segment::free(c);
            } else {
                top.current_child = Some(i);
                stack.push(NodeRecord {
                    first_child: c.line_at(0),
                    child_count,
                    height: top.height + 1,
                    on_boundary: false,
                    current_child: None,
                });
                return;
            }
        }
        i = i + 1;
    }
    stack.pop();
}

pub fn collision_children(node: &NodeRecord, has_vals: u32) {
    for i in 0..node.child_count {
        let idx = (i << 1) as i32;
        let key_count = node.first_child[idx].u32();
        let c = node.first_child[idx + 1].segment();
        if c.unalias() == 0 {
            c.at(0..(key_count << has_vals)).retire();
            Segment::free(c);
        }
    }
}

