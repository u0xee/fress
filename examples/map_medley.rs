// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate ahash;
use ahash::{AHashSet, AHashMap};
extern crate rustc_hash;
use rustc_hash::{FxHashMap, FxHashSet};

extern crate fress;
use fress::*;
use memory::segment::new_free_counts;
use transduce::Process;
use std::env;
use std::collections::{HashSet, HashMap};

static WORDS:     &str = include_str!("../doc/words-shuf.txt");
const WORD_COUNT: usize = 84_035;
static WORDS_500: &str = include_str!("../doc/words-500.txt");

fn main() {
    let args: Vec<String> = env::args().collect();
    let which = &args[1];
    let mut outer: u32 = args[2].parse().unwrap();
    let mut count: usize = args[3].parse().unwrap();
    let sizes = [(200_000, 10),
                 (100_000, 20),
                 ( 50_000, 40),
                 ( 30_000, 60),
                 ( 20_000, 100),
                 ( 10_000, 200),
                 (  5_000, 400),
                 (  3_000, 600),
                 (  2_000, 1000),
                 (  1_000, 2000),
                 (    500, 4000),
                 (    250, 8000),
                 (    100, 16000),
                 (     50, 32000),
                 (     20, 64000),
                 (     10, 100_000),
                 (      5, 200_000),
                 (      3, 400_000),
                 (      1, 1_000_000),
                 (      1, 2_000_000),
                 (      1, 5_000_000),
                 (      1, 10_000_000),
    ];
    if outer == 0 {
        let (o, c) = sizes[count];
        outer = o;
        count = c;
    }
    println!("{}: outer: {}, count: {}", which, outer, count);

    if which == "map-medley" {
        for _ in 0..outer {
            let res = map_medley(count);
        }
    }
    else if which == "set-medley" {
        for _ in 0..outer {
            let res = set_medley(count);
        }
    }
    else if which == "hashmap-medley" {
        for _ in 0..outer {
            let res_len = hashmap_medley(count);
        }
    }
    else if which == "hashset-medley" {
        for _ in 0..outer {
            let res_len = hashset_medley(count);
        }
    }
    else if which == "map-history" {
        let mut hist_count: isize = args[4].parse().unwrap();
        if hist_count < 0 {
            hist_count = (3 * count as isize) / (-hist_count) + 1;
        }
        let mut total_snapshots: u64 = 0;
        for _ in 0..outer {
            let res = map_history2(count, hist_count as usize);
            total_snapshots += res.len() as u64;
        }
        println!("map-history: {} snapshots each (every {} ops) for {} runs",
                 total_snapshots / outer as u64, hist_count, outer);
        println!("total-snapshots {}", total_snapshots);
    }
    else if which == "set-grow" {
        for _ in 0..outer { set_grow(count); }
    }
    else if which == "map-grow" {
        for _ in 0..outer { map_grow(count); }
    }
    else if which == "hashset-grow" {
        for _ in 0..outer { hashset_grow(count, args.len()); }
    }
    else if which == "ahashset-grow" {
        for _ in 0..outer { ahashset_grow(count, args.len()); }
    }
    else if which == "fhashset-grow" {
        for _ in 0..outer { fhashset_grow(count, args.len()); }
    }
    else if which == "hashmap-grow" {
        for _ in 0..outer { hashmap_grow(count, args.len()); }
    }
    else if which == "ahashmap-grow" {
        for _ in 0..outer { ahashmap_grow(count); }
    }
    else if which == "fhashmap-grow" {
        for _ in 0..outer { fhashmap_grow(count); }
    }
    /*
    else if which == "vector-history2" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        let hist_count: usize = args[6].parse().unwrap();
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
    else if which == "vector-medley" {
        let step: u32 = args[4].parse().unwrap();
        let step2: u32 = args[5].parse().unwrap();
        let keep: u32 = args[6].parse().unwrap();
        //let mut prev = vec![];
        for _ in 0..outer {
            // prev.push(vector_medley(count, step, step2, keep, false));
            let res = vector_medley(count, step, step2, keep, false);
        }
    }
    */
    else {
        panic!("Bad choice. Use set-grow, hashset-grow, map-grow or hashmap-grow.");
    }
    let (count_new, count_free) = new_free_counts();
    assert_eq!(count_new, count_free);
}

fn words() -> impl Iterator<Item = Value> {
    use std::iter::repeat;
    WORDS.split_whitespace().map(|w| Value::from(w))
         .chain(WORDS.split_whitespace().cycle()
                .zip((1..).flat_map(|n| repeat(n).take(WORD_COUNT)))
                .map(|(w, i)| Value::from(format!("{}{}", w, i).as_str())))
}
fn words_strings() -> impl Iterator<Item = String> {
    use std::iter::repeat;
    WORDS.split_whitespace().map(|w| format!("{}", w))
         .chain(WORDS.split_whitespace().cycle()
                .zip((1..).flat_map(|n| repeat(n).take(WORD_COUNT)))
                .map(|(w, i)| format!("{}{}", w, i)))
}
fn absent_words() -> impl Iterator<Item = Value> {
    use std::iter::repeat;
    WORDS.split_whitespace().map(|w| Value::from(format!("_{}", w).as_str()))
         .chain(WORDS.split_whitespace().cycle()
                .zip((1..).flat_map(|n| repeat(n).take(WORD_COUNT)))
                .map(|(w, i)| Value::from(format!("_{}{}", w, i).as_str())))
}
fn absent_words_strings() -> impl Iterator<Item = String> {
    use std::iter::repeat;
    WORDS.split_whitespace().map(|w| format!("_{}", w))
         .chain(WORDS.split_whitespace().cycle()
                .zip((1..).flat_map(|n| repeat(n).take(WORD_COUNT)))
                .map(|(w, i)| format!("_{}{}", w, i)))
}

#[inline(never)]
fn set_grow(count: usize) {
    let mut s = hash_set();
    for w in words().take(count) {
        s = s.conj(w);
    }
}
#[inline(never)]
fn hashset_grow(count: usize, args_len: usize) {
    // s.insert, s.contains, s.remove
    let mut s = if args_len > 4 { HashSet::with_capacity(count) }
        else { HashSet::new() };
    for w in words_strings().take(count) {
        s.insert(w);
    }
}
#[inline(never)]
fn ahashset_grow(count: usize, args_len: usize) {
    let mut s = if args_len > 4 { AHashSet::with_capacity(count) }
        else { AHashSet::new() };
    for w in words_strings().take(count) {
        s.insert(w);
    }
}
#[inline(never)]
fn fhashset_grow(count: usize, args_len: usize) {
    let mut s = FxHashSet::default();
    if args_len > 4 {
        s.reserve(count);
    }
    for w in words_strings().take(count) {
        s.insert(w);
    }
}
#[inline(never)]
fn map_grow(count: usize) {
    let mut m = hash_map();
    for (i, w) in words().take(count).enumerate() {
        m = m.assoc(w, i.into());
    }
}
#[inline(never)]
fn hashmap_grow(count: usize, args_len: usize) {
    // m.insert(k,v) m.contains_key, m.remove
    //let mut m = HashMap::with_capacity(count);
    let mut m = if args_len > 4 { HashMap::with_capacity(count) }
        else { HashMap::new() };
    for (i, w) in words_strings().take(count).enumerate() {
        m.insert(w, Value::from(i));
    }
}
#[inline(never)]
fn ahashmap_grow(count: usize) {
    let mut m = AHashMap::new();
    for (i, w) in words_strings().take(count).enumerate() {
        m.insert(w, Value::from(i));
    }
}
#[inline(never)]
fn fhashmap_grow(count: usize) {
    let mut m = FxHashMap::default();
    for (i, w) in words_strings().take(count).enumerate() {
        m.insert(w, Value::from(i));
    }
}
#[inline(never)]
fn map_history2(count: usize, hist_count: usize) -> Value {
    let mut total_ops = 0;
    let mut ops = 0;
    let mut hist = vector();
    let mut m = hash_map();
    while total_ops < 1_000_000 {
        for (i, w) in words().take(count).enumerate() {
            m = m.assoc(w, i.into());
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
        }
        let mut x = Value::from(0);
        for w in words().take(count) {
            m = m.assoc(w, x.value());
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
        }
        for w in words().take(count) {
            m = m.dissoc(&w);
            ops += 1; total_ops += 1;
            if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
        }
    }
    hist = hist.conj(m);
    hist
}
#[inline(never)]
fn map_history(count: usize, hist_count: usize) -> Value {
    let mut ops = 0;
    let mut hist = vector();
    let mut m = hash_map();
    for (i, w) in words().take(count).enumerate() {
        m = m.assoc(w, i.into());
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
    }
    //ops = 0;
    //println!("Done associng");
    let mut x = Value::from(0);
    let rounds = 0;
    for _ in 0..rounds {
        for w in words().take(count) {
            x = x + m[w].value();
        }
    }
    let ans = ((count * (count - 1)) >> 1) * rounds;
    assert_eq!(x, ans.into());
    //println!("Done summing {}", ans);
    let mut ws = words();
    //for i in 0..(count >> 2) {
    for i in 0..count {
        m = m.assoc(ws.next().unwrap(), x.value());
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
    }
    //println!("Done overwriting");
    //ops = 0;
    let rounds = 0;
    for _ in 0..rounds {
        for w in absent_words().take(count) {
            assert!(m[w].is_nil());
        }
    }
    //println!("Done failed lookups");
    //for w in ws.take(count >> 2) {
    for w in words().take(count) {
        m = m.dissoc(&w);
        ops += 1;
        if ops == hist_count { ops = 0; hist = hist.conj(m.value()); }
    }
    //println!("Done dissocing");
    hist = hist.conj(m);
    hist
}
#[inline(never)]
fn map_medley2(count: usize) -> Value {
    let mut m = hash_map();
    for w in absent_words().take(count >> 1) {
        m = m.assoc(w, 0.into());
    }
    let mut ws = words().take(count).enumerate();
    for _ in 0..(count >> 1) {
        let (i, w) = ws.next().unwrap();
        m = m.assoc(w, i.into());
    }
    for w in absent_words().take(count >> 1) {
        m = m.dissoc(&w);
    }
    for (i, w) in ws {
        m = m.assoc(w, i.into());
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words().take(count) {
            x = x + m[w].value();
        }
    }
    let ans = ((count * (count - 1)) >> 1) * rounds;
    assert_eq!(x, ans.into());
    let mut ws = words();
    for _ in 0..(count >> 2) {
        m = m.assoc(ws.next().unwrap(), x.value());
    }
    for w in absent_words().take(count) {
        assert!(m[w].is_nil());
    }
    for w in ws.take(count >> 2) {
        m = m.dissoc(&w);
    }
    m
}
#[inline(never)]
fn map_medley(count: usize) -> Value {
    let mut m = hash_map();
    for (i, w) in words().take(count).enumerate() {
        m = m.assoc(w, i.into());
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words().take(count) {
            x = x + m[w].value();
        }
    }
    let ans = ((count * (count - 1)) >> 1) * rounds;
    assert_eq!(x, ans.into());
    let mut ws = words();
    for _ in 0..(count >> 2) {
        m = m.assoc(ws.next().unwrap(), x.value());
    }
    for w in absent_words().take(count) {
        assert!(m[w].is_nil());
    }
    for w in ws.take(count >> 2) {
        m = m.dissoc(&w);
    }
    m
}
#[inline(never)]
fn set_medley(count: usize) -> usize {
    let mut s = hash_set();
    for w in words().take(count) {
        s = s.conj(w);
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words().take(count) {
            if s.contains(&w) {
                x = x.inc();
            }
        }
    }
    for w in absent_words().take(count) {
        if s.contains(&w) {
            x = x.inc();
        }
    }
    let ans = count * rounds;
    assert_eq!(x, ans.into());
    for w in words().take(count >> 2) {
        s = s.dissoc(&w);
    }
    for w in absent_words().take(count >> 2) {
        s = s.conj(w);
    }
    s.len()
}
#[inline(never)]
fn hashset_medley(count: usize) -> usize {
    //let mut s = HashSet::new();
    //let mut s = AHashSet::new();
    let mut s = FxHashSet::default();
    s.reserve(count);
    for w in words_strings().take(count) {
        s.insert(w);
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words_strings().take(count) {
            if s.contains(&w) {
                x = x.inc();
            }
        }
    }
    for w in absent_words_strings().take(count) {
        if s.contains(&w) {
            x = x.inc();
        }
    }
    let ans = count * rounds;
    assert_eq!(x, ans.into());
    for w in words_strings().take(count >> 2) {
        s.remove(&w);
    }
    for w in absent_words_strings().take(count >> 2) {
        s.insert(w);
    }
    s.len()
}
#[inline(never)]
fn hashmap_medley2(count: usize) -> usize {
    //let mut m = HashMap::new();
    let mut m = AHashMap::new();
    //let mut m = FxHashMap::default();
    //m.reserve(count);
    for w in absent_words_strings().take(count >> 1) {
        m.insert(w, Value::from(0));
    }
    let mut ws = words_strings().take(count).enumerate();
    for _ in 0..(count >> 1) {
        let (i, w) = ws.next().unwrap();
        m.insert(w, Value::from(i));
    }
    for w in absent_words_strings().take(count >> 1) {
        m.remove(&w);
    }
    for (i, w) in ws {
        m.insert(w, Value::from(i));
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words_strings().take(count) {
            x = x + m[&w].value();
        }
    }
    let ans = ((count * (count - 1)) >> 1) * rounds;
    assert_eq!(x, ans.into());
    let mut ws = words_strings();
    for _ in 0..(count >> 2) {
        m.insert(ws.next().unwrap(), x.value());
    }
    for w in absent_words_strings().take(count) {
        assert!(m.get(&w).is_none());
    }
    for w in ws.take(count >> 2) {
        m.remove(&w);
    }
    m.len()
}
#[inline(never)]
fn hashmap_medley(count: usize) -> usize {
    //let mut m = HashMap::new();
    //let mut m = AHashMap::new();
    let mut m = FxHashMap::default();
    //m.reserve(count);
    for (i, w) in words_strings().take(count).enumerate() {
        m.insert(w, Value::from(i));
    }
    let mut x = Value::from(0);
    let rounds = 4usize;
    for _ in 0..rounds {
        for w in words_strings().take(count) {
            x = x + m[&w].value();
        }
    }
    let ans = ((count * (count - 1)) >> 1) * rounds;
    assert_eq!(x, ans.into());
    let mut ws = words_strings();
    for _ in 0..(count >> 2) {
        m.insert(ws.next().unwrap(), x.value());
    }
    for w in absent_words_strings().take(count) {
        assert!(m.get(&w).is_none());
    }
    for w in ws.take(count >> 2) {
        m.remove(&w);
    }
    m.len()
}
#[inline(never)]
fn vector_medley_history2(count: usize, step: u32, step2: u32,
                          hist_count: usize) -> Value {
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
    for i in (0..count as u32).step_by(step2 as usize) {
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
fn vector_medley_history(count: usize, step: u32, step2: u32,
                         hist_count: usize) -> Vec<Value> {
    let mut ops = 0;
    let mut hist = vec![];
    let mut v = vector();
    for i in 0..count {
        v = v.conj(Value::from(i));
        ops += 1;
        if ops == hist_count { ops = 0; hist.push(v.value()); }
    }
    ops = 0;
    let mut x = Value::from(0);
    for i in (0..count as i32).step_by(step as usize) {
        x = x + v[i].value();
    }
    for i in (0..count as u32).step_by(step2 as usize) {
        v = v.nth_set(i, x.value());
        ops += 1;
        if ops == hist_count { ops = 0; hist.push(v.value()); }
    }
    ops = 0;
    for _ in 0..(count - (count >> 2)) {
        let (w, _) = v.pop();
        v = w;
        ops += 1;
        if ops == hist_count { ops = 0; hist.push(v.value()); }
    }
    hist.push(v);
    hist
}
#[inline(never)]
fn vector_medley(count: usize, step: u32, step2: u32, keep: u32, dummy: bool) -> Value {
    let mut v = vector();
    for i in 0..count {
        v = v.conj(if dummy { nil() } else { Value::from(i) });
    }
    let mut x = Value::from(0);
    if !dummy {
        for i in (0..count as i32).step_by(step as usize) {
            x = x + v[i].value();
        }
    }
    for i in (0..count as u32).step_by(step2 as usize) {
        v = v.nth_set(i, x.value());
    }
    for _ in 0..(count - (count >> keep)) {
        let (w, _) = v.pop();
        v = w;
    }
    v
}
#[inline(never)]
fn vec_medley(count: usize, step: u32, step2: u32, keep: u32, dummy: bool) -> Vec<Value> {
    let mut v = vec![];
    for i in 0..count {
        v.push(if dummy { nil() } else { Value::from(i) });
    }
    let mut x = Value::from(0);
    if !dummy {
        for i in (0..count).step_by(step as usize) {
            x = x + v[i].value();
        }
    }
    for i in (0..count).step_by(step2 as usize) {
        v[i] = x.value();
    }
    for _ in 0..(count - (count >> keep)) {
        v.pop();
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

