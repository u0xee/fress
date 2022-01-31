// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use std::fmt;
use memory::*;
use dispatch::*;
use value::Value;
use handle::Handle;
use std::cmp::Ordering;

pub mod guide;
use self::guide::Guide;

// Str abstraction:
// byte buffer (utf8 characters), fast append (tail like vector)
// rope like tree, maybe rrb tree. buffer tree nodes labeled with character count, byte count.

pub struct String_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<String_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_string(h: Handle) -> bool { find_prism(h).is_some() }

pub fn blank(units: u32) -> Guide {
    let needed = 1 /*prism*/ + Guide::units() + units;
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, prism_unit());
    let guide = Guide::hydrate_top_bot(prism, 0, 0);
    for i in 0..(units as i32) {
        guide.root.set(i, Unit::zero());
    }
    guide
}
pub fn new_from_str(source: &str) -> Handle {
    let bytes = source.len() as u32;
    let guide = blank(units_for(bytes)).set_count(bytes);
    guide.byte_slice(bytes).copy_from_slice(source.as_bytes());
    guide.store().segment().unit().handle()
}

pub fn new_escaping(source: &[u8]) -> Result<Handle, String> {
    let bytes = source.len();
    let guide = blank(units_for(bytes as u32));
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
pub fn new_value_from_str(source: &str) -> Value { new_from_str(source).value() }

pub fn units_for(byte_count: u32) -> u32 {
    let (b, c) = if cfg!(target_pointer_width = "32") { (4, 2) } else { (8, 3) };
    (byte_count + b - 1) >> c
}

pub fn byte_slice(prism: &AnchoredLine) -> &[u8] {
    use std::slice::from_raw_parts;
    let guide = Guide::hydrate(*prism);
    let b = guide.byte_slice(guide.count);
    unsafe {
        from_raw_parts(b.as_ptr(), b.len())
    }
}

impl Dispatch for String_ { /*default tear_down, alias_components*/ }
impl Identification for String_ {
    fn type_name(&self) -> &'static str { "String" }
}

impl Distinguish for String_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let h = {
            use random::PI;
            use hash::{mix_range, end};
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
        if let Some(o_str) = find_prism(o) {
            //log!("String eq: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_str);
            return g.str() == h.str()
        } else {
            false
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_str) = find_prism(o) {
            log!("String cmp: {} {}", prism.segment().unit().handle(), o);
            let g = Guide::hydrate(prism);
            let h = Guide::hydrate(o_str);
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
impl Aggregate for String_ { }
impl Sequential for String_ { }
impl Associative for String_ { }
impl Reversible for String_ { }
impl Sorted for String_ { }
impl Notation for String_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide = Guide::hydrate(prism);
        // TODO clojure doesn't support escaped single quotes like \'
        // https://doc.rust-lang.org/src/core/fmt/mod.rs.html#1956-1974
        write!(f, "{:?}", guide.str())
    }
}
impl Numeral for String_ {}
impl Callable for String_ {}

#[cfg(test)]
mod tests {
    use super::*;

}
