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
pub mod dispatch;
pub mod edn;
pub mod float_point;
pub mod fressian;
pub mod fuzz;
pub mod handle;
pub mod hash;
pub mod integral;
pub mod keyword;
pub mod list;
pub mod map;
pub mod memory;
pub mod random;
pub mod range;
pub mod rational;
pub mod set;
pub mod sorted_map;
pub mod sorted_set;
pub mod string;
pub mod symbol;
pub mod transducer;
pub mod value;
pub mod vector;

pub use value::Value;

pub fn nil()  -> Value { Value::nil() }
pub fn tru()  -> Value { Value::tru() }
pub fn fals() -> Value { Value::fals() }
pub fn not(v: &Value) -> Value { !v }
pub fn split(v: Value) -> (Value, Value) { v.split() }
pub fn split_out(v: &Value) -> Value { v.split_out() }
pub fn hash(v: &Value) -> u32 { v.hash() }
pub fn compare(a: &Value, b: &Value) -> Option<std::cmp::Ordering> { unimplemented!() }

pub fn count(c: &Value) -> u32 { c.count() }
pub fn meta(c: &Value) -> &Value { c.meta() }
pub fn with_meta(c: Value, m: Value) -> Value { c.with_meta(m) }
pub fn empty(c: &Value) -> Value { c.empty() }
pub fn conj(c: Value, v: Value) -> Value { c.conj(v) }
pub fn pop(c: Value) -> (Value, Value) { c.pop() }
pub fn peek(c: &Value) -> &Value { c.peek() }
pub fn nth(c: &Value, idx: u32) -> &Value { c.nth(idx) }
pub fn assoc(c: Value, k: Value, v: Value) -> Value { c.assoc(k, v) }
pub fn dissoc(c: Value, k: &Value) -> Value { c.dissoc(k) }
pub fn contains(c: &Value, k: &Value) -> bool { c.contains(k) }
pub fn get<'a>(c: &'a Value, k: &Value) -> &'a Value { c.get(k) }

pub fn inc(x: Value) -> Value { x.inc() }
pub fn dec(x: Value) -> Value { x.dec() }
pub fn neg(x: Value) -> Value { -x }
pub fn add(x: Value, y: Value) -> Value { x + y }
pub fn sub(x: Value, y: Value) -> Value { x - y }
pub fn mul(x: Value, y: Value) -> Value { x * y }
pub fn div(x: Value, y: Value) -> Value { x / y }
pub fn rem(x: Value, y: Value) -> Value { x % y }
pub fn modulus(x: Value, y: Value) -> Value { x.modulus(y) }
pub fn shl(x: Value, shift: u32) -> Value { x << shift }
pub fn shr(x: Value, shift: u32) -> Value { x >> shift }

pub fn vector() -> Value { vector::Vector::new_value() }
pub fn list() -> Value { list::List::new_value() }
pub fn set() -> Value { set::Set::new_value() }
pub fn map() -> Value { map::Map::new_value() }



pub mod agg {
    pub use super::{count, meta, with_meta, empty, conj, pop,
                    peek, nth, assoc, dissoc, contains, get};
    pub use super::{vector, list, set, map};
}
pub mod num {
    pub use super::{inc, dec, neg, add, sub, mul, div, rem, modulus, shl, shr};
}

#[cfg(test)]
mod tests {
    use super::*;

}
