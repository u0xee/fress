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
pub struct Handle {
    pub unit: Unit,
}

impl Handle {
    pub const NIL: Unit = Unit { word: 0x07 };
    pub const TRUE: Unit = Unit { word: !0x00usize };
    pub const FALSE: Unit = Unit { word: !0x08usize };

    pub fn nil() -> Handle {
        Handle::from(Handle::NIL)
    }

    pub fn tru() -> Handle {
        Handle::from(Handle::TRUE)
    }

    pub fn fals() -> Handle {
        Handle::from(Handle::FALSE)
    }

    pub fn value(self) -> Value {
        Value { handle: self }
    }

    pub fn unit(self) -> Unit {
        self.unit
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

    pub fn eq(self, other: Handle) -> bool {
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

    pub fn contains(self, k: Handle) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).contains(prism, k.unit())
        } else {
            unimplemented!()
        }
    }

    pub fn conj(self, x: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).conj(prism, x.unit).handle()
        } else {
            unimplemented!()
        }
    }

    pub fn pop(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (v, _) = mechanism::as_dispatch(&p).pop(prism);
            v.handle()
        } else {
            unimplemented!()
        }
    }

    pub fn assoc(self, k: Handle, v: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (v, replaced) = mechanism::as_dispatch(&p).assoc(prism, k.unit, v.unit);
            replaced.handle().retire();
            v.handle()
        } else {
            unimplemented!()
        }
    }

    pub fn dissoc(self, k: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let v = mechanism::as_dispatch(&p).dissoc(prism, k.unit);
            v.handle()
        } else {
            unimplemented!()
        }
    }

    pub fn get(self, k: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let v = mechanism::as_dispatch(&p).get(prism, k.unit);
            v.handle()
        } else {
            unimplemented!()
        }
    }

    pub fn nth(self, idx: u32) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).nth(prism, idx);
            elem.handle()
        } else {
            unimplemented!()
        }
    }

    pub fn num(x: u32) -> Handle {
        let y = x << 4;
        Unit::from(y | 1).handle()
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).edn(prism, f)
        } else {
            if self.unit == Handle::NIL {
                write!(f, "nil")
            } else if self.unit == Handle::FALSE {
                write!(f, "false")
            } else if self.unit == Handle::TRUE {
                write!(f, "true")
            } else {
                write!(f, "{}", self.unit.u32() >> 4)
            }
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).debug(prism, f)
        } else {
            write!(f, "Handle[0x{:X}]", self.unit.u())
        }
    }
}

impl From<Unit> for Handle {
    fn from(u: Unit) -> Self {
        Handle { unit: u }
    }
}

impl From<Value> for Handle {
    fn from(v: Value) -> Self {
        let ret = v.handle;
        use std::mem::forget;
        forget(v);
        ret
    }
}

