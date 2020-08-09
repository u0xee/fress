// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::char;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use handle::Handle;
use std::fmt;

pub struct Character_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Character_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_character(h: Handle) -> bool { find_prism(h).is_some() }

pub fn new(c: char) -> Handle {
    let s = Segment::new(1 /*prism*/ + if cfg!(target_pointer_width = "32") { 2 } else { 1 });
    s.set(0, prism_unit());
    store(s.line_at(0), 0, c as u32);
    s.unit().handle()
}
pub fn from_byte(b: u8) -> Handle {
    new(char::from_u32(b as u32).unwrap())
}
pub fn from_four_hex(s: &[u8]) -> Handle { new(four_hex_to_char(s)) }

pub fn display(c: Unit, f: &mut fmt::Formatter) -> fmt::Result {
    let ch = char::from_u32((c.u() >> 4) as u32).unwrap();
    match ch {
        '\n' => { write!(f, "\\newline") },
        '\r' => { write!(f, "\\return") },
        ' '  => { write!(f, "\\space") },
        '\t' => { write!(f, "\\tab") },
        _    => { write!(f, "\\{}", ch)}
    }
}

pub fn store(prism: AnchoredLine, hash: u32, c: u32) {
    if cfg!(target_pointer_width = "32") {
        prism.set(1, Unit::from(hash));
        prism.set(2, Unit::from(c));
    } else {
        let x = ((hash as u64) << 32) | c as u64;
        prism.set(1, Unit::from(x));
    }
}

pub fn store_hash(prism: AnchoredLine, hash: u32, c: u32) {
    if cfg!(target_pointer_width = "32") {
        prism.store_hash(1, Unit::from(hash));
        prism.store_hash(2, Unit::from(c));
    } else {
        let x = ((hash as u64) << 32) | c as u64;
        prism.store_hash(1, Unit::from(x));
    }
}

pub fn hydrate(prism: AnchoredLine) -> (u32, u32) {
    assert!(is_prism(prism));
    if cfg!(target_pointer_width = "32") {
        (prism[1].u32(), prism[2].u32())
    } else {
        let x = prism[1].u64();
        ((x >> 32) as u32, x as u32)
    }
}

pub fn four_hex_to_char(s: &[u8]) -> char {
    let code = (hex_digit(s[0]) << 12) | (hex_digit(s[1]) << 8) |
        (hex_digit(s[2]) << 4) | hex_digit(s[3]);
    char::from_u32(code).unwrap()
}

fn lowercase_hex(b: u8) -> u8 { if b <= b'Z' { b + 32 } else { b } }

fn hex_digit(b: u8) -> u32 {
    if b <= b'9' {
        (b - b'0') as u32
    } else {
        (lowercase_hex(b) - b'a') as u32 + 10
    }
}

pub fn as_char(prism: AnchoredLine) -> char {
    assert!(is_prism(prism));
    let (_h, c) = hydrate(prism);
    char::from_u32(c).unwrap()
}

impl Dispatch for Character_ { /*default tear_down, alias_components*/ }
impl Identification for Character_ {
    fn type_name(&self) -> &'static str { "Character" }
}
impl Notation for Character_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let (_, c) = hydrate(prism);
        let ch = char::from_u32(c).unwrap();
        match ch {
            '\n' => { write!(f, "\\newline") },
            '\r' => { write!(f, "\\return") },
            ' '  => { write!(f, "\\space") },
            '\t' => { write!(f, "\\tab") },
            _    => { write!(f, "\\{}", ch) }
        }
    }
}
impl Distinguish for Character_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let (h, c) = hydrate(prism);
        if (h >> 31) & 0x1 == 0x1 {
            return h
        }
        use hash::hash_64;
        let h = hash_64(c as u64, 4) | (1 << 31);
        store_hash(prism, h, c);
        h
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_char) = find_prism(o) {
            log!("Character eq");
            let (_, c) = hydrate(prism);
            let (_, d) = hydrate(o_char);
            return c == d
        }
        false
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_char) = find_prism(o) {
            log!("Character eq");
            let (_, c) = hydrate(prism);
            let (_, d) = hydrate(o_char);
            let ch = char::from_u32(c).unwrap();
            let dh = char::from_u32(d).unwrap();
            return Some(ch.cmp(&dh))
        }
        if o.is_ref() {
            let o_prism_unit = o.logical_value()[0];
            Some(prism_unit().cmp(&o_prism_unit))
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl Aggregate for Character_ { }
impl Sequential for Character_ { }
impl Associative for Character_ { }
impl Reversible for Character_ { }
impl Sorted for Character_ { }
impl Numeral for Character_ { }
impl Callable for Character_ { }

#[cfg(test)]
mod tests {
    use super::*;
}
