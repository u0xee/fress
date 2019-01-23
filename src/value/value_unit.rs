// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
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

    pub fn type_name(self) -> String {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).type_name()
        } else {
            let v = self.unit.u();
            if !(v | 0x8) == 0x0 {
                return "Boolean".to_string()
            }
            if v == 0x7 {
                return "Nil".to_string()
            }
            if (v & 0xF) == 0x3 {
                return "Character".to_string()
            }
            if (v & 0xF) == 0x1 {
                return "Integral".to_string()
            }
            if (v & 0xF) == 0x9 {
                return "FloatPoint".to_string()
            }
            unreachable!("Bad value unit!: 0x{:016X}", v)
        }
    }

    pub fn type_sentinel(self) -> *const u8 {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).type_sentinel()
        } else {
            (self.unit.u() & 0xF) as *const u8
        }
    }

    pub fn tear_down(self) {
        let prism = self.prism();
        let p = prism[0];
        mechanism::as_dispatch(&p).tear_down(prism);
    }

    pub fn eq(self, other: ValueUnit) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).eq(prism, other.unit)
        } else {
            self.unit == other.unit
        }
    }

    pub fn hash(self) -> u32 {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).hash(prism)
        } else {
            use hash::hash_64;
            hash_64(self.unit.u64(), 8)
        }
    }

    pub fn count(self) -> u32 {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).count(prism)
        } else {
            unimplemented!()
        }
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

    pub fn pop(self) -> ValueUnit {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (v, _) = mechanism::as_dispatch(&p).pop(prism);
            v.value_unit()
        } else {
            unimplemented!()
        }
    }

    pub fn assoc(self, k: ValueUnit, v: ValueUnit) -> ValueUnit {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (v, replaced) = mechanism::as_dispatch(&p).assoc(prism, k.unit, v.unit);
            replaced.value_unit().retire();
            v.value_unit()
        } else {
            unimplemented!()
        }
    }

    pub fn nth(self, idx: u32) -> ValueUnit {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).nth(prism, idx);
            elem.value_unit()
        } else {
            unimplemented!()
        }
    }

    pub fn num(x: u32) -> ValueUnit {
        let y = x << 4;
        Unit::from(y | 1).value_unit()
    }
}

impl fmt::Display for ValueUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).edn(prism, f)
        } else {
            write!(f, "{}", self.unit.u32() >> 4)
        }
    }
}

impl fmt::Debug for ValueUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).debug(prism, f)
        } else {
            write!(f, "{}", self.unit.u32() >> 4)
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
