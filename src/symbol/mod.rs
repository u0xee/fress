// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use handle::Handle;

pub mod guide;
use self::guide::Guide;

pub struct Symbol_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Symbol_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_symbol(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new(name: &[u8], solidus_position: u32) -> Unit {
    //log!("Symbol new {}", from_utf8(name).unwrap());
    let guide = {
        let byte_count = name.len() as u32;
        let content_count = units_for(byte_count);
        let needed = 1 /*prism*/ + Guide::units() + content_count;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, prism_unit());
        let g = Guide::new(prism, solidus_position, byte_count);
        g.root.set(content_count as i32 - 1, Unit::zero());
        g.byte_slice().copy_from_slice(name);
        g
    };
    guide.store().segment().unit()
}

pub fn new_prefix_name(prefix: &[u8], name: &[u8]) -> Unit {
    let b = format!("{}/{}", from_utf8(prefix).unwrap(), from_utf8(name).unwrap());
    new(b.as_bytes(), prefix.len() as u32)
}

pub fn has_namespace(prism: AnchoredLine) -> bool {
    assert!(is_prism(prism));
    let guide = Guide::hydrate(prism);
    guide.solidus != 0
}

// TODO what role would a Symbol struct play?
//  Symbol -> Value, Value -> Option<Symbol>
//  Symbol.has_namespace()
//  Symbol.as_str()

// TODO primitives for viewing segment memory as eg, &[u8] &str
// as_str, just namespace, just name
/*
pub fn as_str(prism: &AnchoredLine) -> &str {
    let prism = *prism;
    assert!(is_prism(prism));
    let guide = Guide::hydrate(prism);
    let s = guide.str();
}
*/

pub fn units_for(byte_count: u32) -> u32 {
    let (b, c) = if cfg!(target_pointer_width = "32") { (4, 2) } else { (8, 3) };
    (byte_count + b - 1) >> c
}

impl Dispatch for Symbol_ { /*default tear_down, alias_components*/ }
impl Identification for Symbol_ {
    fn type_name(&self) -> &'static str { "Symbol" }
}
//#[inline(never)]
fn assert_zero(x: usize) -> u32 {
    //println!("x is {}", x);
    //assert_eq!(!0, x ^ !0);
    //assert_eq!(!0, !x);
    //assert_eq!(0, x * 2);
    //assert_eq!(64, x.count_zeros());
    //assert_eq!(0, x.rotate_left(0));
    //println!("x {}", x);
    //assert_eq!(!0, x ^ !0);
    //println!("x leading_zeros {}", x.leading_zeros());
    x.rotate_left(17).leading_zeros()
}
impl Distinguish for Symbol_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix_range, end};
            let iv: (u64, u64, u64, u64) = (PI[14], PI[15], PI[16], PI[17]);
            let unit_count = units_for(guide.count);
            let a = mix_range(guide.root.span(unit_count), iv);
            let (x, _y) = end(a.0, a.1, a.2, a.3);
            x as u32
        };
        //take_my_args(0);
        //log!("Symbol hash: {} {:#08X}", prism.segment().unit().handle(), h);
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let self_usize = self as *const Symbol_ as usize;
        //let x = self_usize ^ 4;
        //assert_eq!(64, x.leading_zeros());
        let z = assert_zero(self_usize);
        println!("z = {}", z);
        //assert_eq!(!0, self_usize ^ !0);
        //assert_zero(self_usize);

        //let self_lead = self_usize.leading_zeros();
        //println!("self_lead {}", self_lead);

        //let x: usize = "1".parse().unwrap();
        //let x_lead = x.leading_zeros();
        //println!("x_lead {}", x_lead);
        //unimplemented!();
        //let f: f64 = Unit::from(self_usize).into();
        //let u: usize = Unit::from(f).into();
        //assert_eq!(0, self_usize.reverse_bits());
        //assert_eq!(0, self as *const Symbol_ as usize);
        let o = other.handle();
        if let Some(o_sym) = find_prism(o) {
            //log!("Symbol eq: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_sym);
            return g.byte_slice() == h.byte_slice()
        } else {
            false
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_sym) = find_prism(o) {
            //log!("Symbol cmp: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_sym);
            Some(g.str().cmp(&h.str()))
        } else {
            if o.is_ref() {
                let o_prism_unit = o.logical_value()[0];
                Some(prism_unit().cmp(&o_prism_unit))
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}
impl Aggregate for Symbol_ { }
impl Sequential for Symbol_ { }
impl Associative for Symbol_ { }
impl Reversible for Symbol_ { }
impl Sorted for Symbol_ { }
impl Notation for Symbol_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        write!(f, "{}", guide.str())
    }
}
impl Numeral for Symbol_ { }
impl Callable for Symbol_ { }

#[cfg(test)]
mod tests {
    use super::*;
}

