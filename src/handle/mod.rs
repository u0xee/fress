// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use std::cmp;
use memory::*;
use dispatch::*;
use value::*;
use transduce::{Transducer, Transducers, Process};

#[derive(Copy, Clone)]
pub struct Handle {
    pub unit: Unit,
}

pub static STATIC_NIL: Unit = Handle::NIL;

impl From<Unit> for Handle {
    fn from(u: Unit) -> Self { Handle { unit: u } }
}

impl From<Value> for Handle {
    fn from(v: Value) -> Self {
        let ret = v.handle;
        use std::mem::forget;
        forget(v);
        ret
    }
}

impl Handle {
    pub fn unit(self) -> Unit { self.unit }
    pub fn segment(self) -> Segment { self.unit.segment() }
    pub fn value(self) -> Value { Value { handle: self } }

    pub const NIL: Unit = Unit { word: 0x07 };
    pub const TRUE: Unit = Unit { word: !0x00usize };
    pub const FALSE: Unit = Unit { word: !0x08usize };

    pub fn nil() -> Handle { Handle::from(Handle::NIL) }
    pub fn tru() -> Handle { Handle::from(Handle::TRUE) }
    pub fn fals() -> Handle { Handle::from(Handle::FALSE) }

    pub fn is_nil(self) -> bool { self.unit == Handle::NIL }
    pub fn is_true(self) -> bool { self.unit == Handle::TRUE }
    pub fn is_false(self) -> bool { self.unit == Handle::FALSE }
    pub fn is_bool(self) -> bool { self.unit.u() | 0x8 == !0 }
    pub fn is_flag(self) -> bool { self.unit.u() & 0x7 == 0x7 } // bool or nil
    pub fn is_not(self) -> bool { self.unit.u() & 0xF == 0x7 }
    pub fn is_so(self) -> bool { !self.is_not() }

    fn tag(self) -> usize { self.unit.u() & 0xF }
    pub fn is_imm_char(self) -> bool { self.tag() == 0x3 }
    pub fn is_imm_int(self) -> bool { self.tag() == 0x1 }
    pub fn is_imm_float(self) -> bool { self.tag() == 0x9 }
}

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

impl Handle {
    pub fn is_ref(self) -> bool { self.unit.is_even() }
    pub fn split(self) { if self.is_ref() { self.segment().alias() } }
    pub fn retire(self) {
        if self.is_ref() && self.segment().unalias() == 0 {
            self.tear_down()
        }
    }
    pub fn prism(self) -> AnchoredLine { self.segment().line_at(0) }

    pub fn tear_down(self) {
        let prism = self.prism();
        let p = prism[0];
        mechanism::as_dispatch(&p).tear_down(prism);
    }

    pub fn unaliased(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).unaliased(prism).handle()
        } else { unimplemented!() }
    }

    pub fn type_sentinel(self) -> *const u8 {
        if self.is_ref() {
            let prism = self.logical_value();
            let p = prism[0];
            mechanism::as_dispatch(&p).type_sentinel()
        } else {
            self.tag() as *const u8
        }
    }

    pub fn type_name(self) -> &'static str {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).type_name()
        } else {
            if self.is_nil()       { return "Nil" }
            if self.is_bool()      { return "Boolean" }
            if self.is_imm_char()  { return "Character" }
            if self.is_imm_int()   { return "Integral" }
            if self.is_imm_float() { return "FloatPoint" }
            unreachable!("Bad handle unit!: 0x{:016X}", self.unit.u())
        }
    }

    pub fn logical_value(self) -> AnchoredLine {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).logical_value(prism)
        } else { unimplemented!() }
    }

    pub fn eq(self, other: Handle) -> bool {
        if self.unit == other.unit { return true; }
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).eq(prism, other.unit)
        } else if other.is_ref() {
            let prism = other.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).eq(prism, self.unit)
        } else {
            false
        }
    }

    pub fn cmp(self, other: Handle) -> Option<cmp::Ordering> {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).cmp(prism, other.unit)
        } else if other.is_ref() {
            let prism = other.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).cmp(prism, self.unit)
        } else {
            self.unit.partial_cmp(&other.unit)
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

    pub fn meta(self) -> *const Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).meta(prism);
            elem as *const Handle
        } else { unimplemented!() }
    }

    pub fn with_meta(self, m: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).with_meta(prism, m.unit()).handle()
        } else { unimplemented!() }
    }

    pub fn invoke1(self, a: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).invoke1(prism, a.unit()).handle()
        } else { unimplemented!() }
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).edn(prism, f)
        } else {
            if self.is_flag() {
                let name = if self.is_nil() { "nil" }
                    else if self.is_true() { "true" } else { "false" };
                write!(f, "{}", name)
            } else if self.is_imm_int() {
                let x: i64 = self.unit.into();
                write!(f, "{}", x >> 4)
            } else if self.is_imm_float() {
                unimplemented!()
            } else if self.is_imm_char() {
                use character::Character;
                Character::display(self.unit, f)
            } else {
                unreachable!("Bad handle unit!: 0x{:016X}", self.unit.u())
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
            write!(f, "{}<", self.type_name());
            fmt::Display::fmt(self, f);
            write!(f, ">")
        }
    }
}

impl Handle {
    pub fn empty(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).empty(prism).handle()
        } else { unimplemented!() }
    }

    pub fn peek(self) -> *const Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).peek(prism);
            elem as *const Handle
        } else { unimplemented!() }
    }

    pub fn count(self) -> u32 {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).count(prism)
        } else { unimplemented!() }
    }

    pub fn contains(self, k: Handle) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).contains(prism, k.unit())
        } else { unimplemented!() }
    }

    pub fn conj(self, x: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).conj(prism, x.unit).handle()
        } else { unimplemented!() }
    }

    pub fn pop(self) -> (Handle, Handle) {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (c, v) = mechanism::as_dispatch(&p).pop(prism);
            (c.handle(), v.handle())
        } else { unimplemented!() }
    }

    pub fn assoc(self, k: Handle, v: Handle) -> (Handle, Handle) {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let (c, displaced) = mechanism::as_dispatch(&p).assoc(prism, k.unit, v.unit);
            (c.handle(), displaced.handle())
        } else { unimplemented!() }
    }

    pub fn dissoc(self, k: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let v = mechanism::as_dispatch(&p).dissoc(prism, k.unit);
            v.handle()
        } else { unimplemented!() }
    }

    pub fn get(self, k: Handle) -> *const Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let v = mechanism::as_dispatch(&p).get(prism, k.unit);
            v as *const Handle
        } else { unimplemented!() }
    }

    pub fn nth(self, idx: u32) -> *const Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).nth(prism, idx);
            elem as *const Handle
        } else { unimplemented!() }
    }

    pub fn reduce(self, stack: &mut [Box<Process>]) -> Value {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).reduce(prism, stack)
        } else { unimplemented!() }
    }

    pub fn pour(self, xf: Transducers, sink: Handle) -> Handle {
        struct Collect {
            c: Handle,
        }
        impl Process for Collect {
            fn ingest   (&mut self, stack: &mut [Box<Process>], v: Value) -> Option<Value> {
                self.c = self.c.conj(Handle::from(v));
                None
            }
            fn ingest_kv(&mut self, stack: &mut [Box<Process>], k: Value, v: Value)
                         -> Option<Value> {
                let (c, displaced) = self.c.assoc(Handle::from(k), Handle::from(v));
                displaced.retire();
                self.c = c;
                None
            }
            fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value { self.c.value() }
        }
        let stack: Vec<Box<Process>> = vec!(Box::new(Collect { c: sink }));
        let mut stack2 = xf.apply(stack);
        Handle::from(self.reduce(&mut stack2))
    }

    pub fn as_i64(self) -> i64 {
        if self.is_ref() {
            if let Some(prism) = self.integral_prism() {
                use integral::Integral;
                Integral::as_i64(prism)
            } else {
                unimplemented!("Can't turn {} into an integer.", self)
            }
        } else {
            if self.is_imm_int() {
                let x: i64 = self.unit.into();
                (x >> 4)
            } else {
                unimplemented!("Can't turn {} into an integer.", self)
            }
        }
    }
}

impl Handle {
    pub fn prism_for(self, sentinel: *const u8) -> Option<AnchoredLine> {
        if self.is_ref() {
            let prism = self.logical_value();
            let p = prism[0];
            if sentinel == mechanism::as_dispatch(&p).type_sentinel() {
                Some(prism)
            } else { None }
        } else { None }
    }

    pub fn integral_prism(self) -> Option<AnchoredLine> {
        use integral::INTEGRAL_SENTINEL;
        self.prism_for((& INTEGRAL_SENTINEL) as *const u8)
    }
    pub fn is_integral(self) -> bool { self.integral_prism().is_some() }

    pub fn string_prism(self) -> Option<AnchoredLine> {
        use string::STR_SENTINEL;
        self.prism_for((& STR_SENTINEL) as *const u8)
    }
    pub fn is_string(self) -> bool { self.string_prism().is_some() }

    pub fn symbol_prism(self) -> Option<AnchoredLine> {
        use symbol::SYMBOL_SENTINEL;
        self.prism_for((& SYMBOL_SENTINEL) as *const u8)
    }
    pub fn is_symbol(self) -> bool { self.symbol_prism().is_some() }

    pub fn list_prism(self) -> Option<AnchoredLine> {
        use list::LIST_SENTINEL;
        self.prism_for((& LIST_SENTINEL) as *const u8)
    }
    pub fn is_list(self) -> bool { self.list_prism().is_some() }

    pub fn vector_prism(self) -> Option<AnchoredLine> {
        use vector::VECTOR_SENTINEL;
        self.prism_for((& VECTOR_SENTINEL) as *const u8)
    }
    pub fn is_vector(self) -> bool { self.vector_prism().is_some() }

    pub fn is_set(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_set()
        } else { false }
    }

    pub fn is_map(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_map()
        } else { false }
    }

    pub fn is_aggregate(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_aggregate()
        } else { false }
    }
}

impl Handle {
    pub fn has_namespace(self) -> bool {
        if let Some(prism) = self.symbol_prism() {
            use symbol::Symbol;
            Symbol::has_namespace(prism)
        } else {
            // keyword
            unimplemented!()
        }
    }
}

impl Handle {
    pub fn add(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let sum = mechanism::as_dispatch(&p).add(prism, rhs.unit);
            sum.handle()
        } else { unimplemented!() }
    }

    pub fn sub(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let diff = mechanism::as_dispatch(&p).subtract(prism, rhs.unit);
            diff.handle()
        } else { unimplemented!() }
    }

    pub fn mul(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let product = mechanism::as_dispatch(&p).multiply(prism, rhs.unit);
            product.handle()
        } else { unimplemented!() }
    }

    pub fn div(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let num = mechanism::as_dispatch(&p).divide(prism, rhs.unit);
            num.handle()
        } else { unimplemented!() }
    }

    pub fn rem(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let num = mechanism::as_dispatch(&p).remainder(prism, rhs.unit);
            num.handle()
        } else { unimplemented!() }
    }

    pub fn modulus(self, rhs: Handle) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let num = mechanism::as_dispatch(&p).modulus(prism, rhs.unit);
            num.handle()
        } else { unimplemented!() }
    }

    pub fn inc(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let x = mechanism::as_dispatch(&p).inc(prism);
            x.handle()
        } else { unimplemented!() }
    }

    pub fn dec(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let x = mechanism::as_dispatch(&p).dec(prism);
            x.handle()
        } else { unimplemented!() }
    }

    pub fn neg(self) -> Handle {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let x = mechanism::as_dispatch(&p).neg(prism);
            x.handle()
        } else { unimplemented!() }
    }

    pub fn bitand(self, rhs: Handle) -> Handle { unimplemented!() }
    pub fn bitor(self, rhs: Handle)  -> Handle { unimplemented!() }
    pub fn bitxor(self, rhs: Handle) -> Handle { unimplemented!() }
    pub fn shl(self, rhs: u32) -> Handle { unimplemented!() }
    pub fn shr(self, rhs: u32) -> Handle { unimplemented!() }
}

