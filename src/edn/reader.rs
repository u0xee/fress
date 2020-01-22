// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use memory::Unit;
use value::Value;
use handle::Handle;

#[derive(Debug)]
pub enum ReadResult {
    Ok       { bytes_used: u32, value: Unit },
    NeedMore { bytes_not_used: u32 },
    Error    { location: Counter, message: String },
}

#[derive(Copy, Clone, Debug)]
pub struct Counter {
    pub byte: u32,
    pub row: u32,
    pub col: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pending {
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

impl Pending {
    pub fn name(self) -> &'static str {
        match self {
            Pending::Vector => "vector",
            Pending::List => "list",
            Pending::Map => "map",
            Pending::Mapping => "mapping pair",
            Pending::Set => "set",
            Pending::Tagged => "tagged",
            Pending::Discard => "discard",
            _ => "pending",
        }
    }
}

pub const STACK_SIZE: usize = 20;

pub struct PendingStack {
    pub count: usize,
    pub discards: usize,
    pub labels: [Pending; STACK_SIZE],
    pub boxes:  [Unit;    STACK_SIZE],
}

pub struct EdnReader {
    pub counter: Counter,
    pub pending: PendingStack,
}

impl EdnReader {
    pub fn new() -> EdnReader {
        EdnReader { counter: Counter::new(), pending: PendingStack::new() }
    }
}

impl Drop for EdnReader {
    fn drop(&mut self) {
        self.pending.tear_down()
    }
}

// TODO file name
pub struct EdnRdr {
    pub reader: EdnReader,
    pub buf: Vec<u8>,
    pub resume: usize,
}

impl EdnRdr {
    pub fn new() -> EdnRdr {
        EdnRdr::with_buffer_capacity(0)
    }

    pub fn with_buffer_capacity(n: usize) -> EdnRdr {
        EdnRdr { reader: EdnReader { counter: Counter::new(), pending: PendingStack::new() },
            buf: Vec::with_capacity(n), resume: 0 }
    }

    pub fn buffer_wilderness(&mut self) -> &mut [u8] {
        use std::slice::from_raw_parts_mut;
        unsafe {
            let b = from_raw_parts_mut(self.buf.as_mut_ptr(), self.buf.capacity());
            &mut b[self.buf.len()..]
        }
    }

    pub fn buffer_consume(&mut self, n: usize) {
        let new_len = self.buf.len() + n;
        assert!(new_len <= self.buf.capacity());
        unsafe { self.buf.set_len(new_len) }
    }

    pub fn align_buffer(&mut self) {
        if self.resume != 0 {
            let byte_count = self.buf.len() - self.resume;
            let b = self.buf.as_mut_slice();
            for i in 0..byte_count {
                b[i] = b[self.resume + i];
            }
            unsafe { self.buf.set_len(byte_count) }
            self.resume = 0;
        }
    }

    pub fn read_bytes(&mut self, s: &[u8]) -> Result<Option<Value>, String> {
        if self.buf.is_empty() {
            let r = super::read(&mut self.reader, s);
            match r {
                ReadResult::Ok { bytes_used, value } => {
                    let remaining = &s[(bytes_used as usize)..];
                    self.buf.reserve(remaining.len());
                    self.buf.extend_from_slice(remaining);
                    Ok(Some(value.handle().value()))
                },
                ReadResult::NeedMore { bytes_not_used } => {
                    let remaining = &s[(s.len() - bytes_not_used as usize)..];
                    self.buf.extend_from_slice(remaining);
                    Ok(None)
                },
                ReadResult::Error { location, message } => {
                    Err(format!("Line {}:{} {}", location.row, location.col, message))
                },
            }
        } else {
            self.buf.reserve(s.len());
            let b = &mut self.buffer_wilderness()[..s.len()];
            b.copy_from_slice(s);
            self.buffer_consume(s.len());
            self.read_again()
        }
    }

    pub fn read_again(&mut self) -> Result<Option<Value>, String> {
        let r = super::read(&mut self.reader, &self.buf.as_slice()[self.resume..]);
        match r {
            ReadResult::Ok { bytes_used, value } => {
                self.resume += bytes_used as usize;
                let remaining = self.buf.len() - self.resume;
                let sixteenth = self.buf.len() >> 4;
                if remaining <= sixteenth { self.align_buffer(); }
                Ok(Some(value.handle().value()))
            },
            ReadResult::NeedMore { bytes_not_used } => {
                self.resume = self.buf.len() - bytes_not_used as usize;
                self.align_buffer();
                Ok(None)
            },
            ReadResult::Error { location, message } => {
                Err(format!("Line {}:{} {}", location.row, location.col, message))
            },
        }
    }
}

impl Counter {
    pub fn new() -> Counter { Counter { byte: 0, row: 1, col: 1 } }

    pub fn count(mut self, s: &str) -> Counter {
        for c in s.chars() {
            if c == '\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        self.byte += s.as_bytes().len() as u32;
        self
    }

    pub fn count_ascii(mut self, s: &[u8]) -> Counter {
        for b in s {
            if *b == b'\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        self.byte += s.len() as u32;
        self
    }

    pub fn newline(mut self) -> Counter {
        self.byte += 1;
        self.row += 1;
        self.col = 1;
        self
    }

    pub fn add_ascii(mut self, n: u32) -> Counter {
        self.byte += n;
        self.col += n;
        self
    }
}

impl PendingStack {
    pub fn new() -> PendingStack {
        PendingStack { count: 0, discards: 0,
            labels: [Pending::Vector; STACK_SIZE],
            boxes:  [Handle::NIL;     STACK_SIZE] }
    }
    pub fn is_empty(&self)    -> bool { self.count == 0 }
    pub fn no_discards(&self) -> bool { self.discards == 0 }
    pub fn push(&mut self, p: Pending, u: Unit) {
        if self.count == STACK_SIZE { panic!("Overfull reader stack") }
        self.labels[self.count] = p;
        self.boxes[self.count] = u;
        self.count += 1;
    }
    pub fn push_discard(&mut self) {
        self.push(Pending::Discard, Handle::NIL);
        self.discards += 1;
    }
    pub fn pop(&mut self) {
        self.count -= 1;
    }
    pub fn pop_discard(&mut self) {
        self.pop();
        self.discards -= 1;
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
        use std::str::from_utf8;
        use string::Str;
        for i in 0..self.count {
            let lab = self.labels[i];
            if lab == Pending::Namespace || lab == Pending::Tagged {
                let ns_unit = self.boxes[i];
                if !ns_unit.is_even() {
                    let resolved = {
                        let (start, length) = demediate_both(ns_unit);
                        let b = &bytes[start..(start + length)];
                        Str::new_from_str(from_utf8(b).unwrap()).unit()
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
        self.discards = 0;
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

