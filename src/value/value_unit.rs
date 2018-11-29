// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
use dispatch::*;
use value::*;

#[derive(Copy, Clone)]
pub struct ValueUnit {
    pub unit: Unit,
}

impl ValueUnit {
    pub fn value(self) -> Value {
        Value::from(self)
    }

    pub fn segment(self) -> Segment {
        self.unit.segment()
    }

    pub fn is_ref(self) -> bool {
        self.unit.is_even()
    }

    pub fn split(self) {
        if self.is_ref() {
            self.segment().alias()
        }
    }

    pub fn retire(self) {
        if self.is_ref() {
            if self.segment().unalias() == 0 {
                self.tear_down()
            }
        }
    }

    pub fn prism(self) -> AnchoredLine {
        self.segment().line_at(0)
    }

    pub fn tear_down(self) {
        let prism = self.prism();
        let p = prism[0];
        mechanism::as_dispatch(&p).tear_down(prism);
    }

    pub fn conj(self, x: ValueUnit) -> ValueUnit {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).conj(prism, x.unit).value_unit()
        } else {
            unimplemented!()
        }
    }
}

impl From<Unit> for ValueUnit {
    fn from(u: Unit) -> Self {
        ValueUnit { unit: u }
    }
}

impl From<Value> for ValueUnit {
    fn from(v: Value) -> Self {
        let ret = ValueUnit { unit: v.handle };
        use std::mem::forget;
        forget(v);
        ret
    }
}

impl ValueUnit {
    pub fn eq(&self, other: Unit) -> bool {
        self.unit == other
    }
}
