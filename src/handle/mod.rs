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
use transduce::{Transducers, Process};
use meta;

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

    pub const NIL:   Unit = Unit { word: 0x07 };
    pub const TRUE:  Unit = Unit { word: !0x00usize };
    pub const FALSE: Unit = Unit { word: !0x08usize };

    pub fn nil()  -> Handle { Handle::from(Handle::NIL) }
    pub fn tru()  -> Handle { Handle::from(Handle::TRUE) }
    pub fn fals() -> Handle { Handle::from(Handle::FALSE) }

    pub fn is_nil(self)   -> bool { self.unit == Handle::NIL }
    pub fn is_true(self)  -> bool { self.unit == Handle::TRUE }
    pub fn is_false(self) -> bool { self.unit == Handle::FALSE }
    pub fn is_bool(self)  -> bool { self.unit.u() | 0x8 == !0 }
    pub fn is_flag(self)  -> bool { self.unit.u() & 0x7 == 0x7 } // bool or nil
    pub fn is_not(self)   -> bool { self.unit.u() & 0xF == 0x7 }
    pub fn is_so(self)    -> bool { !self.is_not() }

    fn tag(self) -> usize { self.unit.u() & 0xF }
    pub fn is_imm_char(self)  -> bool { self.tag() == 0x3 }
    pub fn is_imm_int(self)   -> bool { self.tag() == 0x1 }
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
    pub fn split(self) -> Handle {
        if self.is_ref() { self.segment().alias() }
        self
    }
    pub fn retire(self) {
        if self.is_ref() && self.segment().unalias() == 0 {
            self._tear_down()
        }
    }
    pub fn prism(self) -> AnchoredLine { self.segment().line_at(0) }

    pub fn _tear_down(self) {
        //log!("Handle tearing down 0x{:016X}", self.unit().u());
        mechanism::tear_down(self.prism())
    }
    pub fn _alias_components(self) {
        mechanism::alias_components(self.prism())
    }
    pub fn unaliased(self) -> Handle {
        if self.is_ref() {
            let seg = self.segment();
            if seg.is_aliased() {
                self._alias_components();
                let s = seg.carbon_copy();
                self.retire();
                s.unit().handle()
            } else {
                self
            }
        } else {
            self
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
            mechanism::logical_value(self.prism())
        } else { unimplemented!() }
    }

    pub fn eq(self, other: Handle) -> bool {
        //log!("Handle eq");
        if self.unit == other.unit { return true; }
        if self.is_ref() {
            mechanism::eq(self.prism(), other.unit)
        } else if other.is_ref() {
            mechanism::eq(other.prism(), self.unit)
        } else {
            self.unit == other.unit
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
        //log!("handle hash");
        if self.is_ref() {
            mechanism::hash(self.prism())
        } else {
            use hash::hash_64;
            hash_64(self.unit.u64(), 8)
        }
    }

    // get back, prism ALine and Interface unit
    // now can call Interface methods
    // Or, look in metadata map for ns-qualified symbol, use associated fn
    pub fn meta(self) -> *const Handle {
        meta::get_meta(self)
    }
    pub fn with_meta(self, m: Handle) -> (Handle, Handle) {
        meta::with_meta(self, m)
    }
    pub fn assoc_meta(self, meta_key: Handle, meta_val: Handle) -> Handle {
        meta::assoc_meta(self, meta_key, meta_val)
    }
    pub fn as_immediate(self) -> Handle {
        if self.is_ref() {
            let prism = self.logical_value();
            if meta::is_prism(prism) {
                meta::get_imm(prism)
            } else {
                Handle::nil()
            }
        } else {
            self
        }
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
                use character;
                character::display(self.unit, f)
            } else {
                unreachable!("Bad handle unit!: 0x{:016X}", self.unit.u())
            }
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        meta::do_print_meta();
        let res = fmt::Display::fmt(self, f);
        meta::end_print_meta();
        res
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
        //log!("handle conj");
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

    pub fn assoc(self, k: Handle, v: Handle) -> Handle {
        let (c, displaced) = self.assoc_out(k, v);
        displaced.retire();
        c
    }

    pub fn assoc_out(self, k: Handle, v: Handle) -> (Handle, Handle) {
        //log!("handle assoc on {}", self);
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
        //log!("handle get");
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let v = mechanism::as_dispatch(&p).get(prism, k.unit);
            v as *const Handle
        } else {
            if self.is_nil() {
                (& STATIC_NIL) as *const Unit as *const Handle
            } else {
                unimplemented!()
            }
        }
    }

    pub fn nth(self, idx: u32) -> *const Handle {
        //log!("handle nth");
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            let elem = mechanism::as_dispatch(&p).nth(prism, idx);
            elem as *const Handle
        } else { unimplemented!() }
    }

    pub fn reduce(self, stack: &mut [Box<dyn Process>]) -> Value {
        //log!("handle reduce");
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
            fn ingest   (&mut self, stack: &mut [Box<dyn Process>], v: Value) -> Option<Value> {
                self.c = self.c.conj(v._consume());
                None
            }
            fn ingest_kv(&mut self, stack: &mut [Box<dyn Process>], k: Value, v: Value)
                         -> Option<Value> {
                self.c = self.c.assoc(k._consume(), v._consume());
                None
            }
            fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value { self.c.value() }
        }
        let mut stack = {
            let stack: Vec<Box<dyn Process>> = vec!(Box::new(Collect { c: sink }));
            xf.apply(stack)
        };
        self.reduce(&mut stack)._consume()
    }

    pub fn as_i64(self) -> i64 {
        if self.is_ref() {
            use integral;
            if let Some(prism) = integral::find_prism(self) {
                integral::as_i64(prism)
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
    pub fn find_prism(self, prism_unit: Unit) -> Option<AnchoredLine> {
        if self.is_ref() {
            let prism = self.prism();
            if prism[0] == prism_unit {
                Some(prism)
            } else {
                let prism = self.logical_value();
                if prism[0] == prism_unit {
                    Some(prism)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
    pub fn is_integral(self) -> bool {
        use integral;
        integral::is_integral(self)
    }
    pub fn is_string(self) -> bool {
        use string;
        string::is_string(self)
    }
    pub fn is_symbol(self) -> bool {
        use symbol;
        symbol::is_symbol(self)
    }
    pub fn is_list(self) -> bool {
        use list;
        list::is_list(self)
    }
    pub fn is_vector(self) -> bool {
        use vector;
        vector::is_vector(self)
    }
    pub fn is_set(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_set(prism)
        } else { false }
    }
    pub fn is_map(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_map(prism)
        } else { false }
    }
    pub fn is_aggregate(self) -> bool {
        if self.is_ref() {
            let prism = self.prism();
            let p = prism[0];
            mechanism::as_dispatch(&p).is_aggregate(prism)
        } else { false }
    }
}

impl Handle {
    pub fn has_namespace(self) -> bool {
        use symbol;
        if let Some(prism) = symbol::find_prism(self) {
            symbol::has_namespace(prism)
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

impl PartialEq for Handle {
    fn eq(&self, other: &Handle) -> bool { (*self).eq(*other) }
}
impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Handle) -> Option<cmp::Ordering> { self.cmp(*other) }
}

