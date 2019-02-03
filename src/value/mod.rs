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
    /*
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
    */
}
