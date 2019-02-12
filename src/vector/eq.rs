// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn eq(prism: AnchoredLine, other: Unit) -> bool {
    let guide = Guide::hydrate(prism);
    unimplemented!()
}

#[derive(Copy, Clone, Debug)]
pub struct NodeRecord {
    pub first_child: AnchoredLine,
    pub child_count: u32,
    pub height: u32,
    pub on_boundary: bool,
    pub current_child: Option<u32>,
}

pub const BLANK: NodeRecord = NodeRecord {
    first_child: AnchoredLine { seg: Segment { anchor_line: Line {
        line: 0 as * const Unit } }, index: 0 },
    child_count: 0,
    height: 0,
    on_boundary: false,
    current_child: None,
};

pub fn base_case(node: &NodeRecord) {
    for i in 0..node.child_count {
        let a_tail = node.first_child[i as i32].segment();
        if a_tail.unalias() == 0 {
            a_tail.at(0..TAIL_CAP).retire();
            Segment::free(a_tail);
        }
    }
}

pub struct NodeRecordStack {
    records: *const NodeRecord,
    count: u32,
}

impl NodeRecordStack {
    pub fn new(records: *const NodeRecord) -> NodeRecordStack {
        NodeRecordStack { records, count: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn top<'a>(&self) -> &'a mut NodeRecord {
        unsafe {
            &mut *(self.records as *mut NodeRecord).offset((self.count - 1) as isize)
        }
    }

    pub fn push(&mut self, node: NodeRecord) {
        unsafe {
            let n = &mut *(self.records as *mut NodeRecord).offset(self.count as isize);
            *n = node;
            self.count = self.count + 1;
        }
    }

    pub fn pop(&mut self) {
        self.count = self.count - 1;
    }
}

pub fn step(stack: &mut NodeRecordStack, last_tree_index: u32) {
    let top = stack.top();
    if top.height == 2 {
        base_case(top);
        stack.pop();
        return;
    }
    let search_from = match top.current_child {
        Some(i) => {
            let s = top.first_child[i as i32].segment();
            Segment::free(s);
            i + 1
        },
        None => { 0 },
    };
    let mut i = search_from;
    let cap = top.child_count;
    while i < cap {
        let s = top.first_child[i as i32].segment();
        if s.unalias() == 0 {
            let r = child_record(top, last_tree_index, s, i == cap - 1);
            top.current_child = Some(i);
            stack.push(r);
            return;
        }
        i = i + 1;
    }
    stack.pop();
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

pub fn tear_down_tree(guide: Guide, tailoff: u32) {
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
        step(&mut stack, last_tree_index);
    }
    Segment::free(guide.segment());
}

pub fn tear_down_tailed(guide: Guide) {
    let tail = guide.root[-1].segment();
    if tail.unalias() == 0 {
        tail.at(0..tail_count(guide.count)).retire();
        Segment::free(tail);
    }
    let tailoff = tailoff(guide.count);
    if tailoff == TAIL_CAP {
        guide.root.span(TAIL_CAP).retire();
        Segment::free(guide.segment());
    } else {
        tear_down_tree(guide, tailoff);
    }
}

