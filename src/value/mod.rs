// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::cmp;
use std::fmt;
use std::default;
use std::ops;
use std::hash::{Hash, Hasher};
use handle::*;
use transduce::{Transducers, Process};

pub mod operators;
pub mod conversions;

pub struct Value {
    pub handle: Handle,
}

impl Value {
    pub fn nil()  -> Value { Handle::nil().value() }
    pub fn tru()  -> Value { Handle::tru().value() }
    pub fn fals() -> Value { Handle::fals().value() }

    pub fn is_nil(&self)   -> bool { self.handle().is_nil() }
    pub fn is_true(&self)  -> bool { self.handle().is_true() }
    pub fn is_false(&self) -> bool { self.handle().is_false() }
    pub fn is_not(&self)   -> bool { self.handle().is_not() }
    pub fn is_so(&self)    -> bool { !self.is_not() }

    fn consume(self) -> Handle { Handle::from(self) }
    fn handle(&self) -> Handle { self.handle }
    pub fn _consume(self) -> Handle { self.consume() }
    pub fn _handle(&self) -> Handle { self.handle() }

    pub fn split(self) -> (Value, Value) {
        let v = self.consume();
        v.split();
        (v.value(), v.value())
    }

    pub fn split_out(&self) -> Value {
        let v = self.handle();
        v.split();
        v.value()
    }
    pub fn value(&self) -> Value {
        self.split_out()
    }

    pub fn type_name(&self) -> &'static str { self.handle().type_name() }
    pub fn conj(self, x: Value) -> Value { self.consume().conj(x.consume()).value() }
    pub fn pop(self) -> (Value, Value) {
        let (c, x) = self.consume().pop();
        (c.value(), x.value())
    }

    pub fn peek(&self) -> &Value {
        let v = self.handle().peek() as *const Value;
        unsafe { &*v }
    }

    pub fn is_aggregate(&self) -> bool { self.handle().is_aggregate() }
    pub fn is_list(&self) -> bool { self.handle().is_list() }
    pub fn is_vector(&self) -> bool { self.handle().is_vector() }
    pub fn is_set(&self) -> bool { self.handle().is_set() }
    pub fn is_map(&self) -> bool { self.handle().is_map() }
    pub fn is_symbol(&self) -> bool { self.handle().is_symbol() }
    pub fn is_integral(&self) -> bool { self.handle().is_integral() }

    pub fn count(&self) -> u32 { self.handle().count() }
    pub fn len(&self) -> usize { self.count() as usize }
    pub fn is_empty(&self) -> bool { self.count() == 0 }
    pub fn hash(&self) -> u32 { self.handle().hash() }
    pub fn empty(&self) -> Value { self.handle().empty().value() }
    pub fn contains(&self, k: &Value) -> bool { self.handle().contains(k.handle()) }

    pub fn assoc(self, k: Value, v: Value) -> Value {
        self.consume().assoc(k.consume(), v.consume()).value()
    }

    pub fn assoc_out(self, k: Value, v: Value) -> (Value, Value) {
        let (c, displaced) = self.consume().assoc_out(k.consume(), v.consume());
        (c.value(), displaced.value())
    }

    pub fn dissoc(self, k: &Value) -> Value { self.consume().dissoc(k.handle()).value() }

    pub fn get(&self, k: &Value) -> &Value {
        let v = self.handle().get(k.handle()) as *const Value;
        unsafe { &*v }
    }

    pub fn nth(&self, idx: u32) -> &Value {
        let v = self.handle().nth(idx) as *const Value;
        unsafe { &*v }
    }
    pub fn nth_set(self, idx: u32, v: Value) -> Value {
        self.consume().nth_set(idx, v.consume()).value()
    }
    pub fn swap_idx(self, i: u32, j: u32) -> Value {
        self.consume().swap_idx(i, j).value()
    }
    pub fn mth(&self, idx: i32) -> &Value {
        let ct = self.count() as i32;
        let rm = idx % ct;
        let i = if rm < 0 { rm + ct } else { rm };
        self.nth(i as u32)
    }

    pub fn meta(&self) -> &Value {
        let m = self.handle().meta() as *const Value;
        unsafe { &*m }
    }
    pub fn with_meta(self, m: Value) -> Value {
        let (v, prev_meta) = self.consume().with_meta(m.consume());
        prev_meta.retire();
        v.value()
    }
    pub fn assoc_meta(self, meta_key: Value, meta_val: Value) -> Value {
        self.consume().assoc_meta(meta_key.consume(), meta_val.consume()).value()
    }
    pub fn has_namespace(&self) -> bool { self.handle().has_namespace() }

    pub fn inc(self) -> Value { self.consume().inc().value() }
    pub fn dec(self) -> Value { self.consume().dec().value() }
    pub fn modulus(self, divisor: Value) -> Value {
        self.consume().modulus(divisor.consume()).value()
    }

    pub fn reduce(self, stack: &mut [Box<dyn Process>]) -> Value {
        let s = self.consume();
        let ret = s.reduce(stack);
        s.retire();
        ret
    }

    pub fn pour(self, xf: Transducers, sink: Value) -> Value {
        let s = self.consume();
        let ret = s.pour(xf, sink.consume()).value();
        s.retire();
        ret
    }

    pub fn as_i64(&self) -> i64 { self.handle().as_i64() }
}

impl From<Handle> for Value {
    fn from(h: Handle) -> Self { Value { handle: h } }
}
impl Drop for Value {
    fn drop(&mut self) { self.handle().retire(); }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.handle().fmt(f) }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.handle().fmt(f) }
}
// LowerHex, UpperHex, Pointer, Binary, LowerExp, UpperExp
// f: &mut fmt::Formatter
// if f.alternate() { } else { }
// f.sign_aware_zero_pad()
// f.fill(), f.pad(&str), f.precision(), f.width(), f.align()
impl default::Default for Value {
    fn default() -> Self { Value::nil() }
}
impl Clone for Value {
    fn clone(&self) -> Self { self.split_out() }
}
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool { self.handle().eq(other.handle()) }
}
impl Eq for Value {}
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.hash());
    }
}
//impl PartialEq<i64> for Value {
//    fn eq(&self, other: &i64) -> bool { self.as_i64() == *other }
//}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<cmp::Ordering> {
        self.handle().cmp(other.handle())
    }
}
impl<'a> ops::Index<&'a Value> for Value {
    type Output = Value;
    fn index(&self, index: &'a Value) -> &Value { self.get(index) }
}
impl<'a> ops::Index<Value> for Value {
    type Output = Value;
    fn index(&self, index: Value) -> &Value { self.index(&index) }
}
/*
impl<'a> ops::Index<u32> for Value {
    type Output = Value;
    fn index(&self, index: u32) -> &Value { self.nth(index) }
}
*/
impl<'a> ops::Index<i32> for Value {
    type Output = Value;
    fn index(&self, index: i32) -> &Value { self.mth(index) }
}

#[cfg(test)]
mod test {
    use super::*;
}
// Important Traits:
// Drop, Default, Display, Debug, Clone
// math:       + - * / % neg(-)
// logical:    !
// bitwise:    & | ^ << >>
// Index:      v[k]
// PartialEq:  == !=
// PartialOrd: < <= => >
// From: numbers, strings

// derived?
// unsafe impl Send for Value {}
// unsafe impl Sync for Value {}
// index with i32, Value, &str ?
// partialEq with i32, &str
// FromIterator, Extend, IntoIterator<Value>, IntoIterator<&Value>
use std::iter::{FromIterator, IntoIterator};
impl<T: Into<Value>> FromIterator<T> for Value {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = ::vector();
        for t in iter {
            v = v.conj(t.into());
        }
        v
    }
}
impl<K: Into<Value>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut m = ::hash_map();
        for (k, v) in iter {
            m = m.assoc(k.into(), v.into());
        }
        m
    }
}
pub struct ValueIter { }
impl Iterator for ValueIter {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        todo!()
    }
}
impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = ValueIter;
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
/*
 * Value.iter()
 * Value.keys(), Value.values(), into_keys, into_values
 * Value.iter_kv()
impl<'a> IntoIterator for &'a Value {
    type Item = &'a Value;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
*/

//struct MapValue {}
//struct SetValue {}
//struct SortedMapValue {}
//struct SortedSetValue {}
//struct VectorValue {}
//struct ListValue {}
//struct StringValue {}
//struct Boolean {}
//struct Symbol {}
//struct Keyword {}
//struct Integral {}
//struct Rational {}
//struct FloatPoint {}
//struct Instant {}
//struct Uuid {}

