// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use handle::Handle;

// TODO design parsing machinery
// chew up whitespace, table contains bit for that
// gather digit runs, table contains bit for digit (maybe hex digit?)
// gather string runs, looking for:
//   - " (double-quote) end
//   - \ (backslash) escape
//   - (maybe special chars? DEL NULL etc?)
//
// parsing domains:
//  - whitespace (,)
//  - numbers (digits, . e E)
//  - strings (end of string, or escape)
//  - token delimiters (whitespace, [](){} ;)   >ASCII ?

// New parse context:
// chew up whitespace aka find token start, or delimiter (find
// dispatch based on first character:
//  is it a delimiter? start new collection
//  is it a string? collect string contents
//  is it an alphanumeric? collect a token, interpret it (symbol [nil true false], keyword, int, float)
//  backslash (char), hash (dispatch)

// spin through control characters, space and comma.
// string, dispatch char, aggregate controls, digit +-, comment, char, symbol/keyword, invalid
// simple tests: comment, string, char, dispatch char
// varied: aggregate controls, digit +-, symbol/kw
/*

pub enum ReadResult {
    Ok(Value, u32),
    NeedMore,
    Error{ line: u32, description: String },
}

pub struct EdnReader {
    pub pending: Vec<Pending>,
    pub partial: Option<Partial>,
}

impl EdnReader {
    pub fn new() -> EdnReader {
        EdnReader { pending: Vec::new(), partial: None }
    }

    pub fn read(&mut self, bytes: &[u8]) -> ReadResult {
        if self.partial.is_some() {
            unimplemented!()
        }

        let mut bs = bytes;
        let mut curr = 0usize;
        let mut ready = Handle::nil();
        'start: loop { 'ready: loop {
            while whitespace(bs[curr]) {
                curr += 1;
            }
            // dispatch based on first character
        } // ready
            if let Some(p) = self.pending.pop() {
                match p {
                    Pending::List(h)       => {self.pending.push(Pending::List(h.conj(ready)))},
                    Pending::Vector(h)     => {self.pending.push(Pending::Vector(h.conj(ready)))},
                    Pending::Set(h)        => {self.pending.push(Pending::Set(h.conj(ready)))},
                    Pending::Map(h)        => {unreachable!()},
                    Pending::Mapping       => {self.pending.push(Pending::MappingKey(ready))},
                    Pending::MappingKey(h) => {
                        unimplemented!()
                    },
                    Pending::Tagged        => {self.pending.push(Pending::Tag(ready))},
                    Pending::Tag(h)        => {unimplemented!() },
                }
            } else {
                return ReadResult::Ok(ready.value(), curr);
            }
        }
        unimplemented!()
    }

    pub fn finish(&mut self) -> ReadResult {
        let res = {
            let res = self.read(&b" "[..]);
            match res {
                ReadResult::NeedMore => ReadResult::Error { line: 0, description: "".to_string() },
                _ => res,
            }
        };
        self.partial = None;
        self.pending.clear();
        res
    }
}

pub const SPECIAL:   (u64, u64) = (0x0800_030C_0000_0000, 0x2800_0000_3800_0000); // "#();[\]{}
pub const NUM_NAME:  (u64, u64) = (0xF7FF_EC72_0000_0000, 0x17FF_FFFE_87FF_FFFE); // alphanum .*+!-_?$%&=<>|:/
pub const DELIMITER: (u64, u64) = (0x0800_1301_FFFF_FFFF, 0xA800_0000_2800_0000); // WS (),;[]{}

pub fn digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

pub fn whitespace(b: u8) -> bool {
    b.wrapping_add(1) < b'"' || b == b','
}

pub fn ascii(b: u8) -> bool {
    (b & 0x80) == 0x00
}

pub fn get_bit(x: u64, y: u64, idx: u8) -> u32 {
    let z = x ^ y;
    let word_idx = idx & 0x3F;
    let x_ = (x >> word_idx) as u32;
    let z_ = (z >> word_idx) as u32;
    let masked = z_ & (idx as u32 >> 6);
    (masked ^ x_) & 0x01
}

pub enum Partial {
    Byt(Vec<u8>),
    Str(Handle),
}

pub enum Pending {
    List(Handle),
    Vector(Handle),
    Set(Handle),
    Map(Handle),
    Mapping,
    MappingKey(Handle),
    Tagged,
    Tag(Handle),
}

*/
