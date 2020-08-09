// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use memory::Unit;
use handle::Handle;

#[derive(Debug)]
pub enum ReadResult {
    Ok       { bytes_used: u32, value: Unit },
    NeedMore { bytes_not_used: u32 },
    Error    { location: u32, message: String },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pending {
    // arrays
    // map, set, list
    // bytes strings
    // structs
    Vector, // match closing ]
    List,   // match closing )
    Map,    // match closing }
    Mapping, // read next, add mapping to map
    Namespace,
    Set,    // match closing }
    Tagged, // read next, interpret(?) based on tag
    Discard, // read next, retire
    String,
}

pub const STACK_SIZE: usize = 20;

pub struct PendingStack {
    pub count: usize,
    pub labels: [Pending; STACK_SIZE],
    pub boxes:  [Unit;    STACK_SIZE],
}

pub struct FressianReader {
    pub counter: u32,
    // cache
    // structs
    pub pending: PendingStack,
}

impl PendingStack {
    pub fn new() -> PendingStack {
        PendingStack { count: 0,
            labels: [Pending::Vector; STACK_SIZE],
            boxes:  [Handle::NIL;     STACK_SIZE] }
    }
    pub fn is_empty(&self)    -> bool { self.count == 0 }
    pub fn push(&mut self, p: Pending, u: Unit) {
        if self.count == STACK_SIZE { panic!("Overfull reader stack") }
        self.labels[self.count] = p;
        self.boxes[self.count] = u;
        self.count += 1;
    }
    pub fn pop(&mut self) {
        self.count -= 1;
    }
    pub fn top(&self) -> (Pending, Unit) {
        if self.is_empty() { panic!("Empty reader stack") }
        let t = self.count - 1;
        (self.labels[t], self.boxes[t])
    }
    pub fn top_case(&self) -> Pending {
        if self.is_empty() { panic!("Empty reader stack") }
        self.labels[self.count - 1]
    }
    pub fn top_unit(&self) -> Unit {
        if self.is_empty() { panic!("Empty reader stack") }
        self.boxes[self.count - 1]
    }
    pub fn default_ns(&self) -> Option<Unit> {
        if self.count < 2 { return None; }
        let t = self.count - 2;
        if self.labels[t] == Pending::Namespace {
            Some(self.boxes[t])
        } else {
            None
        }
    }
    pub fn set_top(&mut self, u: Unit) {
        self.boxes[self.count - 1] = u;
    }
    pub fn resolve(&mut self, bytes: &[u8]) {
        for i in 0..self.count {
            let lab = self.labels[i];
            if lab == Pending::Namespace || lab == Pending::Tagged {
                let ns_unit = self.boxes[i];
                if !ns_unit.is_even() {
                    let resolved = {
                        let (start, length) = demediate_both(ns_unit);
                        let b = &bytes[start..(start + length)];
                        use string;
                        string::new_from_str(from_utf8(b).unwrap()).unit()
                    };
                    self.boxes[i] = resolved;
                }
            }
        }
    }
    pub fn tear_down(&mut self) {
        for i in 0..self.count {
            self.boxes[i].handle().retire();
        }
        self.count = 0;
    }
}

impl FressianReader {
    pub fn new() -> FressianReader {
        FressianReader { counter: 0, pending: PendingStack::new() }
    }
}

impl Drop for FressianReader {
    fn drop(&mut self) {
        self.pending.tear_down()
    }
}

pub fn immediate(x: usize) -> Unit { Unit::from((x << 1) | 0x1) }
pub fn demediate(x: Unit)  -> usize { x.u() >> 1 }
pub fn immediate_both(x: usize, y: usize) -> Unit {
    let res = immediate((x << 7) | y);
    assert_eq!(demediate_both(res), (x, y));
    res
}
pub fn demediate_both(x: Unit) -> (usize, usize) {
    let y = demediate(x);
    (y >> 7, y & 0x7F)
}
pub fn reference(name: Unit, bytes: &[u8]) -> &[u8] {
    if name.is_even() {
        if name.handle().is_string() {
            use string::guide::Guide;
            let g = Guide::hydrate(name.handle().prism());
            let bs = g.whole_byte_slice();
            use std::slice;
            unsafe { slice::from_raw_parts(bs.as_ptr(), bs.len()) }
        } else { panic!("Referencing non-string.") }
    } else {
        let (start, length) = demediate_both(name);
        &bytes[start..(start + length)]
    }
}

