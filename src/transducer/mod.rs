// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use Value;

pub trait Process {
    fn ingest(&mut self, process_stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
        let (next, rest) = process_stack.split_last_mut().unwrap();
        next.ingest(rest, v)
    }
    fn ingest_kv(&mut self, process_stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
        let (next, rest) = process_stack.split_last_mut().unwrap();
        next.ingest_kv(rest, k, v)
    }
    fn last_call(&mut self, process_stack: &mut [Box<Process>]) -> Value {
        let (next, rest) = process_stack.split_last_mut().unwrap();
        next.last_call(rest)
    }
}

pub trait Transducer {
    fn process(&self) -> Box<Process>;
    fn transduce(&self, mut process_stack: Vec<Box<Process>>) -> Vec<Box<Process>> {
        process_stack.push(self.process());
        process_stack
    }
    fn info(&self) -> Option<String> {
        None
    }
}

use std::sync::Arc;
pub struct Transducers {
    pub stack: Arc<Vec<Arc<Transducer>>>,
}

impl Transducers {
    pub fn new() -> Transducers {
        Transducers { stack: Vec::new().into() }
    }

    pub fn add_transducer(&mut self, t: Arc<Transducer>) {
        Arc::make_mut(&mut self.stack).push(t)
    }
}

pub fn apply(ts: &Transducers, mut process_stack: Vec<Box<Process>>) -> Vec<Box<Process>> {
    for t in ts.stack.iter().rev() {
        process_stack = t.transduce(process_stack);
    }
    process_stack
}

pub fn ingest(process_stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.ingest(rest, v)
}

pub fn ingest_kv(process_stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.ingest_kv(rest, k, v)
}

pub fn last_call(process_stack: &mut [Box<Process>]) -> Value {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.last_call(rest)
}

