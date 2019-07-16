// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn eq(guide: Guide, o_guide: Guide) -> bool {
    if guide.count != o_guide.count { return false }
    if guide.has_hash() && o_guide.has_hash() && guide.hash != o_guide.hash { return false }

    if guide.count <= TAIL_CAP {
        eq_range(guide.root, o_guide.root, guide.count)
    } else {
        eq_tailed(guide, o_guide)
    }
}

pub fn eq_range(a: AnchoredLine, b: AnchoredLine, count: u32) -> bool {
    for i in 0..(count as i32) {
        if !a[i].handle().eq(b[i].handle()) {
            return false
        }
    }
    true
}

#[derive(Copy, Clone, Debug)]
pub struct NodeRecord {
    pub first_child: (AnchoredLine, AnchoredLine),
    pub child_count: u32,
    pub height: u32,
    pub on_boundary: bool,
    pub current_child: Option<u32>,
}

pub fn base_case(node: &NodeRecord) -> bool {
    for i in 0..(node.child_count as i32) {
        let (a, b) = (node.first_child.0[i], node.first_child.1[i]);
        if a != b {
            let (a_line, b_line) = (a.segment().line_at(0), b.segment().line_at(0));
            if !eq_range(a_line, b_line, TAIL_CAP) {
                return false;
            }
        }
    }
    true
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

pub fn step(stack: &mut NodeRecordStack, last_tree_index: u32) -> bool {
    let top = stack.top();
    if top.height == 2 {
        if !base_case(top) {
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
        let (a, b) = (top.first_child.0[i as i32], top.first_child.1[i as i32]);
        if a != b {
            let r = child_record(top, last_tree_index, a.segment(), b.segment(), i == cap - 1);
            top.current_child = Some(i);
            stack.push(r);
            return true;
        }
        i = i + 1;
    }
    stack.pop();
    return true;
}

pub fn child_record(top: &mut NodeRecord, last_tree_index: u32, a: Segment, b: Segment, last_child: bool)
                    -> NodeRecord {
    let mut r = NodeRecord {
        first_child: (a.line_at(0), b.line_at(0)),
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

pub const BLANK: NodeRecord = NodeRecord {
    first_child: (AnchoredLine { seg: Segment { anchor_line: Line {
        line: 0 as * const Unit } }, index: 0 },
                  AnchoredLine { seg: Segment { anchor_line: Line {
                      line: 0 as * const Unit } }, index: 0 }),
    child_count: 0,
    height: 0,
    on_boundary: false,
    current_child: None,
};

pub fn eq_tree(guide: Guide, o_guide: Guide, tailoff: u32) -> bool {
    let last_tree_index = tailoff - 1;
    let stack_space = [BLANK; 8];
    let mut stack = NodeRecordStack::new(stack_space.as_ptr());
    stack.push(NodeRecord {
        first_child: (guide.root, o_guide.root),
        child_count: root_content_count(tailoff),
        height: digit_count(last_tree_index),
        on_boundary: true,
        current_child: None,
    });
    while !stack.is_empty() {
        if !step(&mut stack, last_tree_index) {
            return false
        }
    }
    true
}

pub fn eq_tailed(guide: Guide, o_guide: Guide) -> bool {
    let (tail, o_tail) = (guide.root[-1], o_guide.root[-1]);
    if tail != o_tail {
        if !eq_range(tail.segment().line_at(0), o_tail.segment().line_at(0), tail_count(guide.count)) {
            return false;
        }
    }
    let tailoff = tailoff(guide.count);
    if tailoff == TAIL_CAP {
        eq_range(guide.root, o_guide.root, TAIL_CAP)
    } else {
        eq_tree(guide, o_guide, tailoff)
    }
}

