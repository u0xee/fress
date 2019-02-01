// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;

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

pub enum Agg {
    // how to add to collection. closing collection matching.
    List(Value),
    Vector(Value),
    Set(Value),
    Map(Value),
    KeyValue,
}

pub enum ReadResult {
    Ok(Value, u32),
    NeedMore,
    Error(String),
}

pub struct EdnReader {
    pub aggregate_stack: Vec<Agg>,
    pub partial_atomic: Vec<u8>,
}

impl EdnReader {
    pub fn new() -> EdnReader {
        EdnReader { aggregate_stack: Vec::new(), partial_atomic: Vec::new() }
    }

    pub fn read(&mut self, bs: &[u8]) -> ReadResult {
        // if partial atomic in progress, try parsing it now with more input
        'start: loop {
            // chew up ws
            // dispatch based on first character
            // match on dispatch enum

        }
    }
}

