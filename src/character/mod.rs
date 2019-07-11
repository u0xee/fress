// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::*;
use dispatch::*;
use handle::Handle;
use value::Value;
use std::fmt;

pub static CHARACTER_SENTINEL: u8 = 0;

pub struct Character {
    prism: Unit,
}

impl Character {
    pub fn new(c: char) -> Handle {
        let s = Segment::new(1 /*prism*/ + if cfg!(target_pointer_width = "32") { 2 } else { 1 });
        s.set(0, mechanism::prism::<Character>());
        store(s.line_at(0), 0, c as u32);
        s.unit().handle()
    }

    pub fn from_byte(b: u8) -> Handle {
        use std::char::from_u32;
        Character::new(from_u32(b as u32).unwrap())
    }

    pub fn from_four_hex(s: &[u8]) -> Handle {
        Character::new(four_hex_to_char(s))
    }

    pub fn display(c: Unit, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char::from_u32;
        let ch = from_u32((c.u() >> 4) as u32).unwrap();
        match ch {
            '\n' => { write!(f, "\\newline") },
            '\r' => { write!(f, "\\return") },
            ' '  => { write!(f, "\\space") },
            '\t' => { write!(f, "\\tab") },
            _    => { write!(f, "\\{}", ch)}
        }
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

pub fn hydrate(prism: AnchoredLine) -> (u32, u32) {
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
    use std::char::from_u32;
    from_u32(code).unwrap()
}

fn lowercase_hex(b: u8) -> u8 {
    if b <= b'Z' { b + 32 } else { b }
}

fn hex_digit(b: u8) -> u32 {
    if b <= b'9' {
        (b - b'0') as u32
    } else {
        (lowercase_hex(b) - b'a') as u32 + 10
    }
}

impl Dispatch for Character {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }
}

impl Identification for Character {
    fn type_name(&self) -> &'static str {
        "Character"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& CHARACTER_SENTINEL) as *const u8
    }
}

impl Notation for Character {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let (_, c) = hydrate(prism);
        use std::char::from_u32;
        let ch = from_u32(c).unwrap();
        match ch {
            '\n' => { write!(f, "\\newline") },
            '\r' => { write!(f, "\\return") },
            ' '  => { write!(f, "\\space") },
            '\t' => { write!(f, "\\tab") },
            _    => { write!(f, "\\{}", ch) }
        }
    }
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Character[");
        self.edn(prism, f);
        write!(f, "]")
    }
}

use std::cmp::Ordering;
impl Distinguish for Character {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let (h, c) = hydrate(prism);
        if (h >> 31) & 0x1 == 0x1 {
            return h
        }
        use hash::hash_64;
        let h = hash_64(c as u64, 4) | (1 << 31);
        store(prism, h, c);
        h
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let c = self.cmp(prism, other);
        c.unwrap() == Ordering::Equal
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if !o.is_ref() {
            return Some(Ordering::Greater)
        }
        if o.type_sentinel() == (& CHARACTER_SENTINEL) as *const u8 {
            let (_, c) = hydrate(prism);
            let (_, d) = hydrate(o.prism());
            use std::char::from_u32;
            let ch = from_u32(c).unwrap();
            let dh = from_u32(d).unwrap();
            return Some(ch.cmp(&dh))
        }
        let ret = ((& CHARACTER_SENTINEL) as *const u8).cmp(&o.type_sentinel());
        Some(ret)
    }
}

impl Aggregate for Character { }
impl Sequential for Character { }
impl Associative for Character { }
impl Reversible for Character { }
impl Sorted for Character { }
impl Numeral for Character { }
