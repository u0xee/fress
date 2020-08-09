// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use value::Value;
use transduce::{Transducers, Process};
use handle::Handle;
use handle::STATIC_NIL;
use ::hash_map;

// P 0 Meta | P ...
// P 1 Meta | imm
pub struct Meta_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Meta_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> {
    if h.is_ref() {
        let prism = h.prism();
        let p = prism[0];
        mechanism::as_dispatch(&p).meta_value(prism)
    } else {
        None
    }
}
pub fn has_meta(h: Handle) -> bool { find_prism(h).is_some() }
pub fn get_meta(v: Handle) -> *const Handle {
    if let Some(m_prism) = find_prism(v) {
        m_prism.offset(2).line().star() as *const Handle
    } else {
        (& STATIC_NIL) as *const Unit as *const Handle
    }
}
pub fn imm_with_meta(imm: Handle, m: Handle) -> Handle {
    assert!(!imm.is_ref());
    let s = Segment::new(4 /*prism, flag, meta, imm*/);
    s.set(0, prism_unit());
    s.set(1, 1.into());
    // TODO
    log!("imm_with_meta: {} {} as_unit: 0x{:016X}", imm, m, imm.unit().u());
    s.set(2, m.unit());
    s.set(3, imm.unit());
    let h = s.unit().handle();
    h
}
pub fn get_imm(prism: AnchoredLine) -> Handle {
    assert!(is_prism(prism));
    assert_eq!(prism[1], Unit::from(1));
    prism[3].handle()
}
pub fn shim_with_meta(v: Handle, m: Handle) -> Handle {
    assert!(v.is_ref());
    let seg = v.segment();
    let h = {
        let cap = seg.capacity();
        let s = Segment::new(cap + 3);
        seg.at(0..cap).to_offset(s, 3);
        s.set(0, prism_unit());
        s.set(1, 0.into());
        log!("shim_with_meta: {} {} next_prism_unit: 0x{:016X}", v, m, s[3].u());
        s.set(2, m.unit());
        s.unit().handle()
    };
    if seg.is_aliased() {
        v._alias_components();
        v.retire();
    } else {
        assert_eq!(seg.unalias(), 0);
        Segment::free(seg);
    }
    h
}
pub fn with_meta(v: Handle, m: Handle) -> (Handle, Handle) {
    if v.is_ref() {
        if let Some(m_prism) = find_prism(v) {
            let w = v.unaliased();
            let mp = m_prism.with_seg(w.segment());
            let curr_meta = mp[2].handle();
            // TODO
            log!("with_meta: Setting meta unit 0x{:016X}", m.unit().u());
            mp.set(2, m.unit());
            (w, curr_meta)
        } else {
            (shim_with_meta(v, m), Handle::nil())
        }
    } else {
        (imm_with_meta(v, m), Handle::nil())
    }
}
// merge_meta
pub fn assoc_meta(v: Handle, meta_key: Handle, meta_val: Handle) -> Handle {
    if v.is_ref() {
        if let Some(m_prism) = find_prism(v) {
            log!("assoc_meta on v = {} 0x{:016X}", v, v.unit().u());
            let w = v.unaliased();
            let mp = m_prism.with_seg(w.segment());
            let curr_meta = {
                let curr_meta = mp[2].handle();
                if curr_meta.is_nil() { hash_map()._consume() } else { curr_meta }
            };
            let (m, old_value) = curr_meta.assoc_out(meta_key, meta_val);
            assert!(old_value.is_nil()); // TODO Remove
            mp.set(2, m.unit());
            log!("assoc_meta done! 0x{:016X}", w.unit().u());
            w
        } else {
            let m = hash_map().assoc(meta_key.value(), meta_val.value());
            shim_with_meta(v, m._consume())
        }
    } else {
        let m = hash_map().assoc(meta_key.value(), meta_val.value());
        imm_with_meta(v, m._consume())
    }
}

impl Dispatch for Meta_ {
    fn tear_down(&self, prism: AnchoredLine) {
        assert_eq!(0, prism.segment().anchor().aliases());
        log!("Meta tear down");
        prism[2].handle().retire();
        if prism[1].u() == 0 {
            mechanism::tear_down(prism.offset(3))
        } else {
            Segment::free(prism.segment());
        }
    }
    fn alias_components(&self, prism: AnchoredLine) {
        prism[2].handle().split();
        if prism[1].u() == 0 {
            mechanism::alias_components(prism.offset(3))
        }
    }
    fn meta_value(&self, prism: AnchoredLine) -> Option<AnchoredLine> { Some(prism) }
    fn logical_value(&self, prism: AnchoredLine) -> AnchoredLine {
        if prism[1].u() == 0 {
            mechanism::logical_value(prism.offset(3))
        } else {
            prism
        }
    }
}
impl Identification for Meta_ {
    fn type_name(&self) -> &'static str { "Meta" }
}

use std::cell::Cell;
thread_local! {
    pub static PRINT_META: Cell<u32> = Cell::new(0);
}
pub fn do_print_meta() {
    PRINT_META.with(|c| {
        let x = c.get();
        c.set(x + 1)
    });
}
pub fn end_print_meta() {
    PRINT_META.with(|c| {
        let x = c.get();
        assert_ne!(x, 0);
        c.set(x - 1)
    });
}
impl Notation for Meta_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let do_print = PRINT_META.with(|c| c.get());
        if do_print > 0 {
            write!(f, "^{} ", prism[2].handle())?;
        }
        log!("Meta edn:");
        prism.segment().print_bits();
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).edn(next_prism, f)
        } else {
            log!("Meta edn, printing imm 0x{:016X}", prism[3].u());
            write!(f, "{}", prism[3].handle())
        }
    }
}
impl Distinguish for Meta_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        if prism[1].u() == 0 {
            mechanism::hash(prism.offset(3))
        } else {
            prism[3].handle().hash()
        }
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        if prism[1].u() == 0 {
            mechanism::eq(prism.offset(3), other)
        } else {
            prism[3].handle().eq(other.handle())
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).cmp(next_prism, other)
        } else {
            prism[3].handle().cmp(other.handle())
        }
    }
}
impl Aggregate for Meta_ {
    fn is_aggregate(&self, prism: AnchoredLine) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).is_aggregate(next_prism)
        } else {
            false
        }
    }
    fn count(&self, prism: AnchoredLine) -> u32 {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).count(next_prism)
        } else {
            unimplemented!()
        }
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).empty(next_prism)
        } else {
            unimplemented!()
        }
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).conj(next_prism, x)
        } else {
            unimplemented!()
        }
    }
    fn peek(&self, prism: AnchoredLine) -> *const Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).peek(next_prism)
        } else {
            unimplemented!()
        }
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).pop(next_prism)
        } else {
            unimplemented!()
        }
    }
    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).get(next_prism, k)
        } else {
            unimplemented!()
        }
    }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<dyn Process>]) -> Value {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).reduce(next_prism, process)
        } else {
            unimplemented!()
        }
    }
}
impl Sequential for Meta_ {
    fn is_sequential(&self, prism: AnchoredLine) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).is_sequential(next_prism)
        } else {
            false
        }
    }
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).nth(next_prism, idx)
        } else {
            unimplemented!()
        }
    }
}
impl Associative for Meta_ {
    fn is_map(&self, prism: AnchoredLine) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).is_map(next_prism)
        } else {
            false
        }
    }
    fn is_set(&self, prism: AnchoredLine) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).is_set(next_prism)
        } else {
            false
        }
    }
    fn contains(&self, prism: AnchoredLine, x: Unit) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).contains(next_prism, x)
        } else {
            false
        }
    }
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).assoc(next_prism, k, v)
        } else {
            unimplemented!()
        }
    }
    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).dissoc(next_prism, k)
        } else {
            unimplemented!()
        }
    }
}
impl Reversible for Meta_ {
    fn reverse(&self, prism: AnchoredLine) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).reverse(next_prism)
        } else {
            unimplemented!()
        }
    }
}
impl Sorted for Meta_ {
    fn subrange(&self, prism: AnchoredLine, start: Unit, end: Unit) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).subrange(next_prism, start, end)
        } else {
            unimplemented!()
        }
    }
}
impl Numeral for Meta_ {
    fn is_numeral(&self, prism: AnchoredLine) -> bool {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).is_numeral(next_prism)
        } else {
            false
        }
    }
}
impl Callable for Meta_ {
    fn invoke0(&self, prism: AnchoredLine) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).invoke0(next_prism)
        } else {
            unimplemented!()
        }
    }
    fn invoke1(&self, prism: AnchoredLine, a: Unit) -> Unit {
        if prism[1].u() == 0 {
            let next_prism = prism.offset(3);
            let p = next_prism[0];
            mechanism::as_dispatch(&p).invoke1(next_prism, a)
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

