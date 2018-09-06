// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

#[derive(Copy, Clone)]
struct NodeRecord {
    first_child: AnchoredLine,
    child_count: u32,
    height: u32,
    on_boundary: bool,
    current_child: Option<u32>,
}

fn base_case(node: &NodeRecord) {
    for i in 0..node.child_count {
        let mut a_tail: Segment = node.first_child[i as usize].into();
        if a_tail.unalias() == 0 {
            for j in 0..TAIL_CAP {
                ValueUnit::from(a_tail[1 + j]).retire();
            }
            Segment::free(a_tail);
        }
    }
}

struct NodeRecordStack {
    records: *const NodeRecord,
    count: u32,
}

impl NodeRecordStack {
    fn new(records: *const NodeRecord) -> NodeRecordStack {
        NodeRecordStack { records: records, count: 0 }
    }
    fn is_empty(&self) -> bool {
        self.count == 0
    }
    fn top<'a>(&self) -> &'a mut NodeRecord {
        unsafe {
            &mut *(self.records as *mut NodeRecord).offset((self.count - 1) as isize)
        }
    }
    fn push(&mut self, node: NodeRecord) {
        unsafe {
            let n = &mut *(self.records as *mut NodeRecord).offset(self.count as isize);
            *n = node;
            self.count = self.count + 1;
        }
    }
    fn pop(&mut self) {
        self.count = self.count - 1;
    }
}

fn step(stack: &mut NodeRecordStack, last_tree_index: u32) {
    let top = stack.top();
    if top.height == 2 {
        base_case(top);
        stack.pop();
    } else {
        let search_from = match top.current_child {
            Some(i) => {
                let s: Segment = top.first_child[i as usize].into();
                Segment::free(s);
                i + 1
            },
            None => {
                0
            },
        };
        let mut i = search_from;
        let cap = top.child_count;
        while i < cap {
            let mut s: Segment = top.first_child[i as usize].into();
            if s.unalias() == 0 {
                let mut r = NodeRecord {
                    first_child: s.line_with_offset(1),
                    child_count: TAIL_CAP,
                    height: top.height - 1,
                    on_boundary: false,
                    current_child: None,
                };
                if (i == (cap - 1)) && top.on_boundary {
                    r.on_boundary = true;
                    let child_idx = (last_tree_index >> (BITS * (r.height - 1))) & MASK;
                    r.child_count = child_idx + 1;
                }
                top.current_child = Some(i);
                stack.push(r);
                return;
            }
            i = i + 1;
        }
        stack.pop();
    }
}

pub fn tear_down(prism: Line) {
    let guide: Guide = prism[1].into();
    let anchor_gap = guide.prism_to_anchor_gap();
    let mut segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    if segment.unalias() == 0 {
        let count = guide.count();
        let root_gap = guide.guide_to_root_gap();
        let first_root_element = 3 + anchor_gap + root_gap;

        if guide.has_meta() {
            ValueUnit::from(segment[3 /*anchor, prism, guide*/ + anchor_gap + guide.meta_gap()]).retire()
        }
        if count <= TAIL_CAP {
            for i in 0..count {
                ValueUnit::from(segment[first_root_element + i]).retire();
            }
            Segment::free(segment);
        } else {
            let tailoff = (count - 1) & !MASK;
            let tail_count = count - tailoff;
            let mut tail = Segment::from(segment[first_root_element - 1]);
            if tail.unalias() == 0 {
                for i in 0..tail_count {
                    ValueUnit::from(tail[1 + i]).retire();
                }
                Segment::free(tail);
            }
            if tailoff == TAIL_CAP {
                for i in 0..TAIL_CAP {
                    ValueUnit::from(segment[first_root_element + i]).retire();
                }
                Segment::free(segment);
            } else {
                let root_count = root_content_count(tailoff);
                let last_tree_index = tailoff - 1;
                let blank = NodeRecord {
                    first_child: Segment::from(Unit::from(0)).line_with_offset(0),
                    child_count: 0,
                    height: 0,
                    on_boundary: false,
                    current_child: None,
                };
                let stack_space = [blank; 8];
                let mut stack = NodeRecordStack::new(stack_space.as_ptr());
                stack.push(NodeRecord {
                    first_child: segment.line_with_offset(first_root_element),
                    child_count: root_count,
                    height: digit_count(last_tree_index),
                    on_boundary: true,
                    current_child: None,
                });
                while !stack.is_empty() {
                    step(&mut stack, last_tree_index);
                }
                Segment::free(segment);
            }
        }
    }
}

