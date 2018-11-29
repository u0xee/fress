// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

pub mod value_unit;
pub use self::value_unit::ValueUnit;

use memory::*;
use dispatch::*;
use vector::Vector;

#[derive(Debug)]
pub struct Value {
    pub handle: Unit,
}

impl Value {
    pub const NIL: Unit = Unit { word: 0x07 };
    pub const TRUE: Unit = Unit { word: !0x00usize };
    pub const FALSE: Unit = Unit { word: !0x08usize };

    fn consume(self) -> ValueUnit {
        ValueUnit::from(self)
    }

    fn value_unit(&self) -> ValueUnit {
        ValueUnit { unit: self.handle }
    }

    pub fn split(self) -> (Value, Value) {
        let v = self.consume();
        v.split();
        (v.value(), v.value())
    }

    pub fn split_out(&self) -> Value {
        let v = self.value_unit();
        v.split();
        v.value()
    }

    pub fn conj(self, x: Value) -> Value {
        self.consume().conj(x.consume()).value()
    }
}

impl From<ValueUnit> for Value {
    fn from(vu: ValueUnit) -> Self {
        Value { handle: vu.unit }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        self.value_unit().retire();
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        // TODO expand to non-immediates
        self.handle == other.handle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passes() {
        assert!(true)
    }

    #[test]
    fn first() {
        let v = Vector::new_value();
        let v1 = v.conj(1.into());
        let v2 = v1.conj(2.into());
        unimplemented!();
        let w = v2.conj(3.into());

        let (w_, three) = w.pop();
        let (w__, two) = w_.pop();
        let (w___, one) = w__.pop();
        assert_eq!(three, 3.into());
        assert_eq!(two, 2.into());
        assert_eq!(one, 1.into());
    }

    /*
    #[test]
    fn testbed() {
        let x = Value { handle: 7 };

    }

    #[test]
    fn is_immediate() {
        assert!(Value {handle: 7}.is_immediate())
    }

    #[test]
    fn is_not() {
        assert!(Value::NIL.is_not() && Value::FALSE.is_not())
    }

    #[test]
    fn is_so() {
        assert!(Value {handle: 0}.is_so())
    }

    #[test]
    fn is_nil() {
        assert!(Value {handle: 7}.is_nil())
    }

    #[test]
    fn is_true() {
        assert!(Value {handle: !0}.is_true())
    }

    #[test]
    fn is_false() {
        assert!(Value {handle: !0 - 8}.is_false())
    }

    #[test]
    fn is_immediate_number() {
        assert!(Value {handle: 1}.is_immediate_number() &&
            Value {handle: 5}.is_immediate_number())
    }

    #[test]
    fn from_u64() {
        let x: u64 = 17;
        let y: Value = x.into();
        let z: u64 = y.into();
        assert_eq!(x, z)
    }
    */
}
