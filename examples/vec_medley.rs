// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::*;
use memory::segment::new_free_counts;
use transduce::Process;
use std::env;

extern crate rand;

//use std::alloc::System;
extern crate jemallocator;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
//static GLOBAL: System = System;

fn main() {
    let args: Vec<String> = env::args().collect();
    let which = &args[1];
    let mut outer: u32 = args[2].parse().unwrap();
    let count: u32 = args[3].parse().unwrap();

    /*
    let nums: Value = (0..24).map(|x| 2 * x).collect();
    dbg!(nums);
    let pairs: Value = (0..24).zip(24..).collect();
    dbg!(pairs);
    */
    if outer == 0 {
        outer = 10_000_000 / count;
    }

    if which == "vector" {
        for _ in 0..outer {
            vector_grow(count);
        }
        println!("vector");
    }
    else if which == "vector-history2" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        let hist_count: u32 = args[6].parse().unwrap();
        let mut total_snapshots: u64 = 0;
        for _ in 0..outer {
            let res = vector_medley_history2(count, step, step2, hist_count);
            total_snapshots += res.len() as u64;
        }
        let (count_new, count_free) = new_free_counts();
        assert_eq!(count_new, count_free);
        println!("vector-history2: {} snapshots each (every {} ops) for {} runs",
                 total_snapshots / outer as u64, hist_count, outer);
        println!("Count: {}, {} reads (every {}), {} writes (every {})",
                 count, count / step, step, count / step2, step2);
    }
    else if which == "vector-history" {
        let hist_count: usize = args[4].parse().unwrap();
        let use_shuf: bool = args.len() > 5;
        let mut total_snapshots: u64 = 0;

        use rand::prelude::SliceRandom;
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        use random::PI;

        let mut idx:  Vec<u32> = (0..count).collect();
        let mut shuf: Vec<u32> = (0..count).collect();
        shuf.shuffle(&mut StdRng::seed_from_u64(PI[42]));
        for _ in 0..outer {
            let res = vector_medley_history(count, hist_count, &idx, &shuf, use_shuf);
            total_snapshots += res.len() as u64;
        }
        println!("vector-history: {} snapshots each (every {} ops) for {} runs",
                 total_snapshots / outer as u64, hist_count, outer);
        println!("total-snapshots {}", total_snapshots);
    }
    else if which == "vector-medley" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        for _ in 0..outer {
            let res = vector_medley(count, step, step2);
        }
        println!("{}: outer {}, count {}, read {} (every {}), write {} (every {})",
                 which, outer, count, count/step, step, count/step2, step2);
    }
    else if which == "vector-medley-shuffle" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();

        use rand::prelude::SliceRandom;
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        use random::PI;

        let mut idx:  Vec<usize> = (0..count as usize).step_by(step as usize).collect();
        let mut idx2: Vec<usize> = (0..count as usize).step_by(step2 as usize).collect();
        idx.shuffle(&mut StdRng::seed_from_u64(PI[42]));
        idx2.shuffle(&mut StdRng::seed_from_u64(PI[43]));
        for _ in 0..outer {
            let res = vector_medley_shuffle(count, &idx, &idx2);
        }
        println!("{}: outer {}, count {}, read {} (every {}), write {} (every {})",
                 which, outer, count, count/step, step, count/step2, step2);
    }
    else if which == "vec-medley-shuffle" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        let with_cap: bool = args.len() > 6;

        use rand::prelude::SliceRandom;
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        use random::PI;

        let mut idx:  Vec<usize> = (0..count as usize).step_by(step as usize).collect();
        let mut idx2: Vec<usize> = (0..count as usize).step_by(step2 as usize).collect();
        idx.shuffle(&mut StdRng::seed_from_u64(PI[42]));
        idx2.shuffle(&mut StdRng::seed_from_u64(PI[43]));
        for _ in 0..outer {
            let res = vec_medley_shuffle(count, &idx, &idx2, with_cap);
        }
        println!("{}: outer {}, count {}, read {} (every {}), write {} (every {})",
                 which, outer, count, count/step, step, count/step2, step2);
    }
    else if which == "vec-medley" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        let with_cap: bool = args.len() > 6;
        for _ in 0..outer {
            let res = vec_medley(count, step, step2, with_cap);
        }
        println!("{}: outer {}, count {}, read {} (every {}), write {} (every {})",
                 which, outer, count, count/step, step, count/step2, step2);
    }
    else if which == "vec-swap" {
        let mut v = vec![];
        for i in 0..count {
            v.push(Value::from(i));
        }
        for _ in 0..outer {
            v = vec_swap(v);
        }
        println!("vec-swap");
    }
    else if which == "vec" {
        for _ in 0..outer {
            vec_grow(count);
        }
        println!("vec");
    }
    else if which == "vec_cap" {
        for _ in 0..outer {
            vec_grow_with_cap(count);
        }
        println!("vec with capacity");
    }
    else {
        println!("Bad choice. Use vector, vec, or vec_cap");
    }
    let (count_new, count_free) = new_free_counts();
    assert_eq!(count_new, count_free);
}

#[inline(never)]
fn vector_grow(count: u32) {
    let mut v = vector();
    for i in 0..count {
        v = v.conj(Value::from(i));
    }
}
#[inline(never)]
fn vec_grow(count: u32) {
    let mut v = vec![];
    for i in 0..count {
        v.push(Value::from(i));
    }
}
#[inline(never)]
fn vec_grow_with_cap(count: u32) {
    let mut v = Vec::with_capacity(count as usize);
    for i in 0..count {
        v.push(Value::from(i));
    }
}
#[inline(never)]
fn vector_medley_history2(count: u32, step: u32, step2: u32,
                          hist_count: u32) -> Value {
    let mut ops = 0;
    let mut hist = vector();
    let mut v = vector();
    for i in 0..count {
        v = v.conj(Value::from(i));
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(v.value()); }
    }
    ops = 0;
    let mut x = Value::from(0);
    for i in (0..count as i32).step_by(step as usize) {
        x = x + v[i].value();
    }
    for i in (0..count).step_by(step2 as usize) {
        v = v.nth_set(i, x.value());
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(v.value()); }
    }
    ops = 0;
    for i in 0..(count - (count >> 2)) {
        let (w, _) = v.pop();
        v = w;
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(v.value()); }
    }
    hist = hist.conj(v);
    hist
}
#[inline(never)]
fn vector_medley_history(count: u32, hist_count: usize,
                         idx: &[u32], shuf: &[u32],
                         use_shuf: bool) -> Vec<Value> {
    let mut total_ops = 0;
    let mut ops = 0;
    let mut hist = vec![];
    let mut v = vector();
    while total_ops < 1_000_000 {
        for i in 0..count {
            v = v.conj(Value::from(i));
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist.push(v.value()); }
        }
        let mut x = Value::from(0);
        for &i in if use_shuf { shuf } else { idx } {
            v = v.nth_set(i, x.value());
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist.push(v.value()); }
        }
        for _ in 0..count {
            let (w, _) = v.pop();
            v = w;
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist.push(v.value()); }
        }
    }
    hist.push(v);
    hist
}
#[inline(never)]
fn vector_medley(count: u32, step: u32, step2: u32) -> Value {
    let mut v = vector();
    for i in 0..count {
        v = v.conj(Value::from(i));
    }
    let mut x = Value::from(0);
    for i in (0..count as i32).step_by(step as usize) {
        x = x + v[i].value();
    }
    for i in (0..count).step_by(step2 as usize) {
        v = v.nth_set(i, x.value());
    }
    for _ in 0..(count >> 2) {
        let (w, _) = v.pop();
        v = w;
    }
    v
}
#[inline(never)]
fn vector_medley_shuffle(count: u32, idx: &[usize], idx2: &[usize]) -> Value {
    let mut v = vector();
    for i in 0..count {
        v = v.conj(Value::from(i));
    }
    let mut x = Value::from(0);
    for &i in idx {
        x = x + v[i as i32].value();
    }
    for &i in idx2 {
        v = v.nth_set(i as u32, x.value());
    }
    for _ in 0..(count >> 2) {
        let (w, _) = v.pop();
        v = w;
    }
    v
}
#[inline(never)]
fn vec_medley(count: u32, step: u32, step2: u32, with_cap: bool) -> Vec<Value> {
    let mut v = if with_cap {
        Vec::with_capacity(count as usize)
    } else { vec![] };
    for i in 0..count {
        v.push(Value::from(i));
    }
    let mut x = Value::from(0);
    for i in (0..count as usize).step_by(step as usize) {
        x = x + v[i].value();
    }
    for i in (0..count as usize).step_by(step2 as usize) {
        v[i] = x.value();
    }
    for _ in 0..(count >> 2) {
        v.pop();
    }
    v
}
#[inline(never)]
fn vec_medley_shuffle(count: u32, idx: &[usize], idx2: &[usize], with_cap: bool) -> Vec<Value> {
    let mut v = if with_cap {
        Vec::with_capacity(count as usize)
    } else { vec![] };
    for i in 0..count {
        v.push(Value::from(i));
    }
    let mut x = Value::from(0);
    for &i in idx {
        x = x + v[i].value();
    }
    for &i in idx2 {
        v[i] = x.value();
    }
    for _ in 0..(count >> 2) {
        v.pop();
    }
    v
}
#[inline(never)]
fn vector_swap(mut v: Value) -> Value {
    let count = v.count();
    let swaps = count >> 1;
    let x = Value::from(7);
    let y = Value::from(7);
    let z = x + y;
    dbg!(z);

    for i in 0..swaps {
        v = v.swap_idx(i, count - 1 - i);
    }
    v
    /*
    struct Reduce {
        r: Handle,
    }
    impl Process for Reduce {
        fn ingest   (&mut self, stack: &mut [Box<dyn Process>], v:  Value)            -> Option<Value> {
            inges(stack, &v)
        }
        fn last_call(&mut self, _stack: &mut [Box<dyn Process>]) -> Value { self.r.value() }
    }
    let mut stack: Box<dyn Process> = Box::new(Reduce { r: locals_used._consume() });
    use std::slice::from_mut;
    used.reduce(from_mut(&mut stack))
    */
}
#[inline(never)]
fn vec_swap(mut v: Vec<Value>) -> Vec<Value> {
    let count = v.len();
    let swaps = count >> 1;
    for i in 0..swaps {
        v.swap(i, count - 1 - i);
    }
    v
}
/*
fn create_destroy(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..CT {
            let x = Value::from(i);
        }
    });
}
*/

