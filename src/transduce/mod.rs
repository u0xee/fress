// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use std::sync::Arc;
use memory::*;
use dispatch::*;
use Value;

// Model:
// Transducer -> [Transducers] stack
// Process stack [reduce base]
// Transducers stack -> Process Stack -> Ready Process Stack
// Ready Stack ingest value, value, ...
// Last call.

// Transduction contexts:
// reduce
// fold
// iter
// channel

// tree reduce(stack)
// range, eduction reduce(stack)
// fold -> reduce combine

// vector tree, forward and reverse reduce

fn top(stack: &mut [Box<Process>]) -> *mut Box<Process> {
    stack.last_mut().unwrap() as *mut Box<Process>
}

pub fn ingest   (stack: &mut [Box<Process>], v:  Value)            -> Option<Value> {
    unsafe { (*top(stack)).ingest(stack, v) }
}
pub fn inges    (stack: &mut [Box<Process>], v: &Value)            -> Option<Value> {
    unsafe { (*top(stack)).inges(stack, v) }
}
pub fn ingest_kv(stack: &mut [Box<Process>], k:  Value, v:  Value) -> Option<Value> {
    unsafe { (*top(stack)).ingest_kv(stack, k, v) }
}
pub fn inges_kv (stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
    unsafe { (*top(stack)).inges_kv(stack, k, v) }
}
pub fn last_call(stack: &mut [Box<Process>]) -> Value {
    unsafe { (*top(stack)).last_call(stack) }
}

pub trait Process {
    fn ingest   (&mut self, stack: &mut [Box<Process>], v:  Value)            -> Option<Value> {
        inges(stack, &v)
    }
    fn inges    (&mut self, stack: &mut [Box<Process>], v: &Value)            -> Option<Value> {
        ingest(stack, v.split_out())
    }
    fn ingest_kv(&mut self, stack: &mut [Box<Process>], k:  Value, v:  Value) -> Option<Value> {
        inges_kv(stack, &k, &v)
    }
    fn inges_kv (&mut self, stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
        ingest_kv(stack, k.split_out(), v.split_out())
    }
    fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value {
        let (_, rest) = stack.split_last_mut().unwrap();
        last_call(rest)
    }
}

pub struct Pass { }
impl Process for Pass {
    fn ingest   (&mut self, stack: &mut [Box<Process>], v:  Value)            -> Option<Value> {
        let (_, rest) = stack.split_last_mut().unwrap();
        ingest(rest, v)
    }
    fn inges    (&mut self, stack: &mut [Box<Process>], v: &Value)            -> Option<Value> {
        let (_, rest) = stack.split_last_mut().unwrap();
        inges(rest, v)
    }
    fn ingest_kv(&mut self, stack: &mut [Box<Process>], k:  Value, v:  Value) -> Option<Value> {
        let (_, rest) = stack.split_last_mut().unwrap();
        ingest_kv(rest, k, v)
    }
    fn inges_kv (&mut self, stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
        let (_, rest) = stack.split_last_mut().unwrap();
        inges_kv(rest, k, v)
    }
}

/*
pub trait Process_ {
    fn ingest(&mut self, process_stack: &mut [Box<Process_>], v: &Value) -> Option<Value> {
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

pub fn ingest2(process_stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.ingest(rest, v)
}

pub fn ingest2_kv(process_stack: &mut [Box<Process>], k: &Value, v: &Value) -> Option<Value> {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.ingest_kv(rest, k, v)
}

pub fn last_call2(process_stack: &mut [Box<Process>]) -> Value {
    let (top, rest) = process_stack.split_last_mut().unwrap();
    top.last_call(rest)
}

pub struct Pass {}
impl Process for Pass {}

pub struct Map<F, Next> {
    pub function: F,
    pub next: Next,
}

impl<F: Fn(&Value) -> Value, Next: Process> Process for Map<F, Next> {
    fn ingest(&mut self, process_stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
        let x = (self.function)(v);
        self.next.ingest(process_stack, &x)
    }
    fn last_call(&mut self, process_stack: &mut [Box<Process>]) -> Value {
        self.next.last_call(process_stack)
    }
}
*/


pub struct Xf<F> {
    pub new_process: F,
}

impl<F: 'static + Fn() -> Box<Process>> Transduce for Xf<F> {
    fn process(&self) -> Box<Process> {
        (self.new_process)()
    }
}

impl<F: 'static + Fn() -> Box<Process>> Xf<F> {
    pub fn new(f: F) -> Transducer {
        Transducer { t: Arc::new(Xf { new_process: f }) }
    }
}

pub fn filter(pred: fn(&Value) -> bool) -> Transducer {
    struct Filter<G> { pred: G }
    impl<G: Fn(&Value) -> bool> Process for Filter<G> {
        fn ingest   (&mut self, stack: &mut [Box<Process>], v:  Value)            -> Option<Value> {
            if (self.pred)(&v) {
                let (_, rest) = stack.split_last_mut().unwrap();
                ingest(rest, v)
            } else {
                None
            }
        }
        fn inges    (&mut self, stack: &mut [Box<Process>], v: &Value)            -> Option<Value> {
            if (self.pred)(v) {
                let (_, rest) = stack.split_last_mut().unwrap();
                inges(rest, v)
            } else {
                None
            }
        }
    }
    Xf::new(move || Box::new(Filter { pred }))
}

pub trait Transduce {
    fn process(&self) -> Box<Process>;
    fn transduce(&self, mut process_stack: Vec<Box<Process>>) -> Vec<Box<Process>> {
        process_stack.push(self.process());
        process_stack
    }
    fn info(&self) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub struct Transducer {
    pub t: Arc<Transduce>,
}

impl Transducer {
    pub fn process(&self) -> Box<Process> { self.t.process() }
    pub fn transduce(&self, process_stack: Vec<Box<Process>>) -> Vec<Box<Process>> {
        self.t.transduce(process_stack)
    }
}

pub struct Transducers {
    pub stack: Arc<Vec<Transducer>>,
}

impl Transducers {
    pub fn new() -> Transducers {
        Transducers { stack: Vec::new().into() }
    }

    pub fn add_transducer(&mut self, t: Transducer) {
        Arc::make_mut(&mut self.stack).push(t)
    }

    pub fn apply(&self, mut process_stack: Vec<Box<Process>>) -> Vec<Box<Process>> {
        for t in self.stack.iter().rev() {
            process_stack = t.transduce(process_stack);
        }
        process_stack
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello() {

    }
}


