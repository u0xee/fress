// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

mod aggregate;
mod arithmetic;
pub mod distinguish;
mod immediate;

/*
Value is the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- &Value into another Value (during its scope)
- Special high level operations like split
*/

use memory::Unit;
use memory::Segment;
use vector::Vector;
use dispatch::*;


#[derive(Debug)]
pub struct Value {
    pub handle: Unit,
}

use std;
use dispatch::Dispatch;

impl Value {
    pub const NIL: Unit = Unit { word: 0x07 };
    pub const TRUE: Unit = Unit { word: std::usize::MAX };
    pub const FALSE: Unit = Unit { word: !0x08usize };

    pub fn add_one_test(&self) -> Value {
        let x: u64 = self.handle.into();
        Value { handle: (x + 1).into() }
    }

    pub fn conj(self, x: Value) -> Value {
        if self.is_immediate() {
            unimplemented!()
        } else {
            let s = Segment::from(self.handle);
            let next_seg = s.conj(x.handle);
            println!("value::conj = {:?}", Segment::from(next_seg));
            use std::mem::forget;
            forget(self);
            Value { handle: next_seg }
            // TODO dropping self here?
        }
    }

    pub fn pop(self) -> (Value, Value) {
        if self.is_immediate() {
            unimplemented!()
        } else {
            let s = Segment::from(self.handle);
            let (a, b) = s.pop();
            (Value { handle: a }, Value { handle: b })
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if !self.is_immediate() {
            let s = Segment::from(self.handle);
            s.tear_down()
        }
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
