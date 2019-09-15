// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! A cohesive fressian library for rust

pub mod array;
pub mod atom;
pub mod channel;
pub mod character;
pub mod dispatch;
pub mod edn;
pub mod eval;
pub mod float_point;
pub mod fressian;
pub mod fuzz;
pub mod graph;
pub mod handle;
pub mod hash;
pub mod inst;
pub mod integral;
pub mod keyword;
pub mod learn;
pub mod list;
pub mod map;
pub mod memory;
pub mod queue;
pub mod random;
pub mod range;
pub mod rational;
pub mod regex;
pub mod set;
pub mod sort_map;
pub mod sort_set;
pub mod string;
pub mod symbol;
pub mod tagged;
pub mod transduce;
pub mod uri;
pub mod uuid;
pub mod value;
pub mod vector;
pub mod wasm;

pub use value::Value;
pub use transduce::{Transducer, Transducers};

#[no_mangle]
pub extern fn fress_nil() -> usize { 7 }

pub fn nil()  -> Value { Value::nil() }
pub fn tru()  -> Value { Value::tru() }
pub fn fals() -> Value { Value::fals() }
pub fn is_nil(v: &Value) -> bool { v.is_nil() }
pub fn is_true(v: &Value) -> bool { v.is_true() }
pub fn is_false(v: &Value) -> bool { v.is_false() }
pub fn is_not(v: &Value) -> bool { v.is_not() }
pub fn is_so(v: &Value) -> bool { v.is_so() }
pub fn not(v: &Value) -> Value { !v }
pub fn split(v: Value) -> (Value, Value) { v.split() }
pub fn split_out(v: &Value) -> Value { v.split_out() }
pub fn hash(v: &Value) -> u32 { v.hash() }
pub fn eq(a: &Value, b: &Value) -> bool { a == b }
pub fn compare(a: &Value, b: &Value) -> Option<std::cmp::Ordering> { a.partial_cmp(b) }
pub fn type_name(v: &Value) -> &'static str { v.type_name() }

pub fn count(c: &Value) -> u32 { c.count() }
pub fn is_empty(c: &Value) -> bool { c.count() == 0 }
pub fn has_mass(c: &Value) -> bool { !is_empty(c) }
pub fn meta(c: &Value) -> &Value { c.meta() }
pub fn with_meta(c: Value, m: Value) -> Value { c.with_meta(m) }
pub fn empty(c: &Value) -> Value { c.empty() }
pub fn conj(c: Value, v: Value) -> Value { c.conj(v) }
pub fn pop(c: Value) -> (Value, Value) { c.pop() }
pub fn peek(c: &Value) -> &Value { c.peek() }
pub fn nth(c: &Value, idx: u32) -> &Value { c.nth(idx) }
pub fn mth(c: &Value, idx: i32) -> &Value { unimplemented!() } // modular nth
pub fn assoc(c: Value, k: Value, v: Value) -> Value { c.assoc(k, v) }
pub fn dissoc(c: Value, k: &Value) -> Value { c.dissoc(k) }
pub fn contains(c: &Value, k: &Value) -> bool { c.contains(k) }
pub fn get<'a>(c: &'a Value, k: &Value) -> &'a Value { c.get(k) }

pub fn inc(x: Value) -> Value { x.inc() }
pub fn dec(x: Value) -> Value { x.dec() }
pub fn max(x: Value, y: Value) -> Value { unimplemented!() }
pub fn min(x: Value, y: Value) -> Value { unimplemented!() }
pub fn neg(x: Value) -> Value { -x }
pub fn abs(x: Value) -> Value { unimplemented!() }
pub fn is_zero(x: &Value) -> bool { unimplemented!() }
pub fn is_pos(x: &Value) -> bool { unimplemented!() }
pub fn is_neg(x: &Value) -> bool { unimplemented!() }
pub fn is_nat(x: &Value) -> bool { !is_neg(x) }
pub fn add(x: Value, y: Value) -> Value { x + y }
pub fn sub(x: Value, y: Value) -> Value { x - y }
pub fn mul(x: Value, y: Value) -> Value { x * y }
pub fn div(x: Value, y: Value) -> Value { x / y }
pub fn quot(x: Value, y: Value) -> Value { unimplemented!() }
pub fn rem(x: Value, y: Value) -> Value { x % y }
pub fn modulus(x: Value, y: Value) -> Value { x.modulus(y) }
pub fn shl(x: Value, shift: u32) -> Value { x << shift }
pub fn shr(x: Value, shift: u32) -> Value { x >> shift }

pub fn read(source: &str) -> Result<Value, String> { source.parse() }

pub fn vector()   -> Value { vector::Vector::new_value() }
pub fn list()     -> Value { list::List::new_value() }
pub fn hash_map() -> Value { map::Map::new_value() }
pub fn hash_set() -> Value { set::Set::new_value() }
pub fn sort_map() -> Value { unimplemented!() }
pub fn sort_set() -> Value { unimplemented!() }

pub fn subvec(c: Value) -> Value { unimplemented!() }
pub fn union(s: Value, t: Value) -> Value { unimplemented!() }
pub fn difference(s: Value, t: Value) -> Value { unimplemented!() }
pub fn intersection(s: Value, t: Value) -> Value { unimplemented!() }
pub fn is_subset(s: &Value, t: &Value) -> bool { unimplemented!() }
pub fn is_superset(s: &Value, t: &Value) -> bool { unimplemented!() }
pub fn into(sink: Value, xf: Transducers, source: Value) -> Value { source.pour(xf, sink) }
pub fn reduce(red: u32, xf: u32, f: u32) -> Value { unimplemented!() }
pub fn educe(red: u32, xf: u32) -> u32 { unimplemented!() }
pub fn max_key(c: &Value, f: u32) -> &Value { unimplemented!() }
pub fn min_key(c: &Value, f: u32) -> &Value { unimplemented!() }
pub fn zipmap(ks: Value, vs: Value) -> Value { unimplemented!() }
pub fn group_by(f: u32, red: u32) -> Value { unimplemented!() }

pub fn is_number(v: &Value) -> bool { unimplemented!() }
pub fn is_integral(v: &Value) -> bool { unimplemented!() }
pub fn is_rational(v: &Value) -> bool { unimplemented!() }
pub fn is_float_point(v: &Value) -> bool { unimplemented!() }
pub fn is_keyword(v: &Value) -> bool { unimplemented!() }
pub fn is_symbol(v: &Value) -> bool { unimplemented!() }
pub fn is_string(v: &Value) -> bool { unimplemented!() }
pub fn is_boolean(v: &Value) -> bool { unimplemented!() }
pub fn is_char(v: &Value) -> bool { unimplemented!() }
pub fn is_vector(v: &Value) -> bool { unimplemented!() }
pub fn is_list(v: &Value) -> bool { unimplemented!() }
pub fn is_map(v: &Value) -> bool { unimplemented!() }
pub fn is_set(v: &Value) -> bool { unimplemented!() }
pub fn is_hash_map(v: &Value) -> bool { unimplemented!() }
pub fn is_hash_set(v: &Value) -> bool { unimplemented!() }
pub fn is_sort_map(v: &Value) -> bool { unimplemented!() }
pub fn is_sort_set(v: &Value) -> bool { unimplemented!() }
pub fn is_aggregate(v: &Value) -> bool { unimplemented!() }
pub fn is_sequential(v: &Value) -> bool { unimplemented!() }
pub fn is_associative(v: &Value) -> bool { unimplemented!() }
pub fn is_inst(v: &Value) -> bool { unimplemented!() }
pub fn is_uuid(v: &Value) -> bool { unimplemented!() }

pub fn atom(v: Value) -> u64 { unimplemented!() }
pub fn swap(a: u64, f: &Fn(Value) -> Value) -> Value { unimplemented!() }
pub fn reset(a: u64, v: Value) -> Value { unimplemented!() }

pub fn str_new(source: &str) -> Value { source.into() }
pub fn str(s: Value, t: Value) -> Value { unimplemented!() }
pub fn substr(s: Value, r: std::ops::Range<u32>) -> Value { unimplemented!() }
pub fn str_split(s: Value, sep: Value) -> Value { unimplemented!() }
pub fn trim(s: Value) -> Value { unimplemented!() }
pub fn starts_with(s: &Value, t: &Value) -> bool { unimplemented!() }
pub fn ends_with  (s: &Value, t: &Value) -> bool { unimplemented!() }
pub fn name     (s: &Value) -> &str { unimplemented!() }
pub fn namespace(s: &Value) -> &str { unimplemented!() }

pub fn bools(n: u32) -> Value { unimplemented!() }
pub fn i32s(n: u32) -> Value { unimplemented!() }
pub fn i64s(n: u32) -> Value { unimplemented!() }
pub fn f32s(n: u32) -> Value { unimplemented!() }
pub fn f64s(n: u32) -> Value { unimplemented!() }
pub fn arr_sort(a: Value) -> Value { unimplemented!() }
pub fn arr_sort_by(a: Value, key_fn: u32) -> Value { unimplemented!() }
pub fn arr_rotate(a: Value, n: u32) -> Value { unimplemented!() }
pub fn u8s(n: u32) -> Value { unimplemented!() }
pub fn u32s(n: u32) -> Value { unimplemented!() }
pub fn u64s(n: u32) -> Value { unimplemented!() }
pub fn varray(n: u32) -> Value { unimplemented!() }


pub fn filter(pred: fn(&Value) -> bool) -> Transducer { transduce::filter(pred) }
pub fn take(n: u32) -> Transducer { unimplemented!() }
pub fn drop(n: u32) -> Transducer { unimplemented!() }
pub fn range(r: std::ops::Range<i64>) -> Value { unimplemented!() }

// reducible: repeat, cycle, range, iterate, repeatedly
// transducers: keys, vals, map, filter, take, drop, cat, mapcat
// distinct, dedupe, take-nth, concat, interleave, interpose, replace
// take-while, drop-while, take-last, drop-last, sort
// partition, partition-by, split-at, split-with, every? not-every? not-any?

pub fn spit(filename: String, data: &Value) { unimplemented!() }
pub fn slurp(filename: String) -> Value { unimplemented!() }
pub fn sha3(filename: String) -> (u64, u64, u64, u64) { unimplemented!() }
pub fn k12(filename: String, customization: String) -> (u64, u64, u64, u64) { unimplemented!() }

pub fn reduced(v: Value) -> Value { unimplemented!() }
pub fn is_reduced(v: &Value) -> bool { unimplemented!() }

pub fn chan() -> u64 { unimplemented!() }
pub fn chan_take(c: u64) -> Value { unimplemented!() }
pub fn chan_put(c: u64, v: Value) { unimplemented!() }
pub fn alts() { unimplemented!() }
pub fn promise_chan() -> u64 { unimplemented!() }
pub fn offer(c: u64, v: Value) { unimplemented!() }
pub fn poll(c: u64) -> Value { unimplemented!() }

