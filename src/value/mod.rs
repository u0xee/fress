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
use memory::*;
use handle::*;
use dispatch::*;

pub struct Value {
    pub handle: Handle,
}

impl Value {
    fn consume(self) -> Handle {
        Handle::from(self)
    }

    fn handle(&self) -> Handle { self.handle }

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

    pub fn conj(self, x: Value) -> Value {
        self.consume().conj(x.consume()).value()
    }

    pub fn count(&self) -> u32 {
        self.handle().count()
    }

    pub fn hash(&self) -> u32 {
        self.handle().hash()
    }

    pub fn empty(&self) -> Value {
        self.handle().empty().value()
    }

    pub fn contains(&self, k: &Value) -> bool {
        self.handle().contains(k.handle())
    }

    pub fn assoc(self, k: Value, v: Value) -> Value {
        self.consume().assoc(k.consume(), v.consume()).value()
    }

    pub fn dissoc(self, k: &Value) -> Value {
        self.consume().dissoc(k.handle()).value()
    }

    pub fn get(&self, k: &Value) -> &Value {
        let v = self.handle().get(k.handle()) as *const Value;
        unsafe { &*v }
    }

    pub fn nth(&self, idx: u32) -> &Value {
        let v = self.handle().nth(idx) as *const Value;
        unsafe { &*v }
    }
}

impl From<&'static str> for Value {
    fn from(s: &'static str) -> Self {
        // read edn string
        unimplemented!()
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        use integral::Integral;
        Integral::new(x).handle().value()
    }
}

impl From<bool> for Value {
    fn from(x: bool) -> Self {
        unimplemented!()
    }
}

impl From<Handle> for Value {
    fn from(h: Handle) -> Self {
        Value { handle: h }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        self.handle().retire();
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

impl default::Default for Value {
    fn default() -> Self {
        Handle::nil().value()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        self.split_out()
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        self.consume().add(rhs.consume()).value()
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Rem for Value {
    fn rem(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::BitXor for Value {
    type Output = Value;

    fn bitxor(self, rhs: Value) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Shl<u32> for Value {
    type Output = Value;

    fn shl(self, rhs: u32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Shr<u32> for Value {
    type Output = Value;

    fn shr(self, rhs: u32) -> Self::Output {
        unimplemented!()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.handle().eq(other.handle())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<cmp::Ordering> {
        unimplemented!()
    }
}

impl<'a> ops::Index<&'a Value> for Value {
    type Output = Value;

    fn index(&self, index: &'a Value) -> &Value {
        self.get(index)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

// Important Traits:
// Drop, Default, Display, Debug, Clone
// math:    + - * / %
// bitwise: & | ^ << >>
// Index: v[k]
// PartialEq:  == !=
// PartialOrd: < <= => >
// From: numbers, strings

// Value handle - bit patterns:
//
// !(handle | 0x08) => 0, boolean
// 0xFFFFFFFFFFFFFFFF true
// 0xFFFFFFFFFFFFFFF7 false
//
// End in 0b0111, logically negative
// 0x0000000000000007 nil
// 0xFFFFFFFFFFFFFFF7 false
//
// End in 0b011
// 0xXXXXXXXX00000003 char
// 0xXXXXXXXXXXXXXXLB string, L holds count
//
// End in 0b001
// 0xXXXXXXXXXXXXXXX1 integral
// 0xXXXXXXXXXXXXXXX9 FloatPoint
//
// Even handles (rightmost bit of 0) are pointers.
// They point to segments that have a distributor as the first unit.

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

pub fn new_vector() -> Value {
    use vector::Vector;
    Vector::new_value()
}

pub fn new_list() -> Value {
    use list::List;
    List::new_value()
}

pub fn new_map() -> Value {
    use map::Map;
    Map::new_value()
}

pub fn new_set() -> Value {
    use set::Set;
    Set::new_value()
}

