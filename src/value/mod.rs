// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

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

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

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
