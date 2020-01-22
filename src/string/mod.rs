// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use std::fmt;
use std::io;
use memory::*;
use dispatch::*;
use transduce::{Process};
use value::Value;
use handle::Handle;

pub mod guide;
use self::guide::Guide;

pub static STR_SENTINEL: u8 = 0;

// Str abstraction:
// byte buffer (utf8 characters), fast append (tail like vector)
// rope like tree, maybe rrb tree. buffer tree nodes labeled with character count, byte count.

pub struct Str {
    prism: Unit,
}

impl Str {
    pub fn blank(units: u32) -> Guide {
        let needed = 1 /*prism*/ + Guide::units() + units;
        let s = Segment::new(needed);
        let prism = s.line_at(0);
        prism.set(0, mechanism::prism::<Str>());
        let guide = Guide::hydrate_top_bot(prism, 0, 0);
        for i in 0..(units as i32) {
            guide.root.set(i, Unit::zero());
        }
        guide
    }

    pub fn new_from_str(source: &str) -> Handle {
        let bytes = source.len() as u32;
        let guide = Str::blank(units_for(bytes)).set_count(bytes);
        guide.byte_slice(bytes).copy_from_slice(source.as_bytes());
        guide.store().segment().unit().handle()
    }

    pub fn new_escaping(source: &[u8]) -> Result<Handle, String> {
        let bytes = source.len();
        let guide = Str::blank(units_for(bytes as u32));
        let mut fill = 0usize;
        let buf = guide.byte_slice(bytes as u32);
        let mut i = 0;
        while i < bytes {
            let b = source[i];
            let c = if b != b'\\' { b } else {
                i += 1;
                let c = source[i];
                match c {
                    b'\\' => b'\\',
                    b'"'  => b'"',
                    b'\'' => b'\'',
                    b'n'  => b'\n',
                    b'r'  => b'\r',
                    b't'  => b'\t',
                    b'0'  => b'\0',
                    b'u'  => {
                        if i + 4 < bytes {
                            let code = &source[(i + 1)..(i + 5)];
                            use edn::after_base16;
                            if after_base16(code).is_some() {
                                return Err(format!("Bad string escape ({}). A unicode literal should have \
                                                    four hex digits (0-9 a-f A-F), like \\u03BB.",
                                                   from_utf8(code).unwrap()))
                            }
                            use character::four_hex_to_char;
                            let d = four_hex_to_char(code);
                            let dlen = d.encode_utf8(&mut buf[fill..]).len();
                            fill += dlen;
                            i += 5;
                            continue;
                        } else {
                            return Err(format!("Bad string escape ({}). A unicode literal should have \
                                                four hex digits (0-9 a-f A-F), like \\u03BB.",
                                               from_utf8(&source[(i - 1)..]).unwrap()))
                        }
                    },
                    _ => {
                        let seg = guide.segment();
                        seg.unalias();
                        Segment::free(seg);
                        use std::char::from_u32;
                        return Err(format!("Bad string escape (\\{}). To include a backslash in a \
                                            string, use a double backslash (\\\\).",
                                           from_u32(c as u32).unwrap()))
                    }
                }
            };
            buf[fill] = c;
            fill += 1;
            i += 1;
        }
        Ok(guide.set_count(fill as u32).store().segment().unit().handle())
    }

    pub fn new_value_from_str(source: &str) -> Value {
        Str::new_from_str(source).value()
    }
}

pub fn units_for(byte_count: u32) -> u32 {
    let (b, c) = if cfg!(target_pointer_width = "32") { (4, 2) } else { (8, 3) };
    (byte_count + b - 1) >> c
}

impl Dispatch for Str {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        Segment::free(prism.segment())
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
}

impl Identification for Str {
    fn type_name(&self) -> &'static str { "String" }
    fn type_sentinel(&self) -> *const u8 { (& STR_SENTINEL) as *const u8 }
}

use std::cmp::Ordering;
impl Distinguish for Str {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix, mix_range, end};
            let iv: (u64, u64, u64, u64) = (PI[22], PI[23], PI[24], PI[25]);
            let unit_count = units_for(guide.count);
            let a = mix_range(guide.root.span(unit_count), iv);
            let (x, _y) = end(a.0, a.1, a.2, a.3);
            x as u32
        };
        guide.set_hash(h).store_hash().hash
    }

    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if o.is_string() {
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o.prism());
            return g.str() == h.str()
        }
        false
    }

    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if !o.is_ref() {
            return Some(Ordering::Greater)
        }
        if o.type_sentinel() != (& STR_SENTINEL) as *const u8 {
            let ret = ((& STR_SENTINEL) as *const u8).cmp(&o.type_sentinel());
            return Some(ret)
        }
        let g = Guide::hydrate(prism);
        let h = Guide::hydrate(o.prism());
        Some(g.str().cmp(&h.str()))
    }
}

impl Aggregate for Str {
    fn count(&self, prism: AnchoredLine) -> u32 {
        unimplemented!()
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        unimplemented!()
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        unimplemented!()
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        unimplemented!()
    }
    fn reduce(&self, prism: AnchoredLine, process: &mut [Box<Process>]) -> Value {
        unimplemented!()
    }
}
impl Sequential for Str { }
impl Associative for Str { }
impl Reversible for Str {}
impl Sorted for Str {}

impl Notation for Str {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        // TODO clojure doesn't support escaped single quotes like \'
        // https://doc.rust-lang.org/src/core/fmt/mod.rs.html#1956-1974
        write!(f, "{:?}", guide.str())
    }
}

impl Numeral for Str {}
impl Callable for Str {}

#[cfg(test)]
mod tests {
    use super::*;

}
