// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use memory::Unit;
use handle::Handle;
use value::Value;

pub mod number;
pub mod name;
pub mod reader;
use self::reader::{EdnReader, ReadResult, Pending, Counter};


pub fn err(reader: &mut EdnReader, msg: String) -> ReadResult {
    let res = ReadResult::Error { location: reader.counter, message: msg };
    reader.counter.clear();
    reader.pending.tear_down();
    res
}

pub fn more(reader: &mut EdnReader, bytes: &[u8], bytes_not_used: usize) -> ReadResult {
    reader.pending.resolve(bytes);
    ReadResult::NeedMore { bytes_not_used: bytes_not_used as u32 }
}

// profile guided optimization; slow path error branches.
// maybe split out error branches into #[cold] function calls
pub fn read(reader: &mut EdnReader, bytes: &[u8]) -> ReadResult {
    let mut i = 0usize;
    let mut ready = Handle::NIL;
    let mut string_ready = false;
    if !reader.pending.is_empty() && reader.pending.top().0 == Pending::String {
        let partial = reader.pending.top().1;
    }
    'top: loop { 'ready: loop {
        if string_ready { string_ready = false; break 'ready; }
        if i >= bytes.len() {
            return more(reader, bytes, 0)
        }
        let c = bytes[i];
        assert!(ascii(c));
        if hit(c, NUM_NAME) { // alphanum .*+!-_?$%&=<>|:/
            if let Some(sym) = isolate_symbolic(&bytes[i..]) {
                let res = if digit(c) || (sign(c) && sym.len() > 1 && digit(sym[1])) {
                    println!("Number at {:?}", &reader.counter); // TODO
                    number::parse_numeric(sym)
                } else {
                    use edn::reader::reference;
                    let default = reader.pending.default_ns().map(|ns| reference(ns, bytes));
                    println!("Symbolic at {:?}", &reader.counter); // TODO
                    name::parse_symbol_keyword(sym, default)
                };
                match res {
                    Ok(h) => {
                        ready = h.unit();
                        reader.counter.add(sym.len() as u32);
                        i += sym.len();
                        break 'ready;
                    },
                    Err(err_string) => {
                        return err(reader, err_string)
                    },
                }
            } else {
                return more(reader, bytes, bytes.len() - i)
            }
        }
        if hit(c, SPECIALS) { // #,;'"`^@~\
            if c == b'#' {
                if (i + 1) >= bytes.len() {
                    return more(reader, bytes, 1)
                }
                let d = bytes[i + 1];
                assert!(ascii(d));

                if d == b'{' {
                    use set::Set;
                    let h = Set::new();
                    reader.pending.push(Pending::Set, h);
                    reader.counter.add(2);
                    i += 2;
                    continue 'top;
                }
                if d == b':' {
                    match prefix_map(reader, bytes, i) {
                        ReadResult::Ok { bytes_used, .. } => {
                            i += bytes_used as usize;
                            continue 'top;
                        },
                        res => { return res },
                    }
                }
                if hit(d, ALPHABET) {
                    match tagged(reader, bytes, i) {
                        ReadResult::Ok { bytes_used, value } => {
                            i += bytes_used as usize;
                            if value.handle().is_nil() {
                                continue 'top;
                            } else {
                                ready = value;
                                break 'ready;
                            }
                        },
                        res => { return res },
                    }
                }
                if d == b'_' {
                    reader.pending.push_discard();
                    reader.counter.add(2);
                    i += 2;
                    continue 'top;
                }
                if d == b'#' {
                    match symbolic_numbers(reader, bytes, i) {
                        ReadResult::Ok { bytes_used, value } => {
                            ready = value;
                            reader.counter.add(bytes_used);
                            i += bytes_used as usize;
                            break 'ready;
                        },
                        res => { return res },
                    }
                }
                return err(reader, format!("Unknown dispatch character ({})", char::from(d)))
            }
            if c == b'"' {
                let str_start = &bytes[i..];
                println!("String at {:?}", &reader.counter); // TODO
                if let Some(quote_index) = string_end_quote_index(str_start, &mut reader.counter) {
                    use string::Str;
                    match Str::new_escaping(&str_start[1..quote_index]) {
                        Ok(h) => {
                            ready = h.unit();
                            i += quote_index + 1 /*end quote*/;
                            break 'ready;
                        },
                        Err(msg) => {
                            return err(reader, msg)
                        },
                    }
                } else {
                    // |"hello there\|
                    // |"hello th\u03|
                    // copy into new partial string and return NeedMore
                    //reader.partial = Some(Partial::Str(handle));
                    //return ReadResult::NeedMore { bytes_not_used: not_used }
                    return more(reader, bytes, bytes.len() - i)
                }
            }
            if c == b'\\' {
                match character(reader, bytes, i) {
                    ReadResult::Ok { bytes_used, value } => {
                        ready = value;
                        reader.counter.add(bytes_used);
                        i += bytes_used as usize;
                        break 'ready;
                    },
                    res => { return res },
                }
            }
            if c == b';' {
                if let Some(lf) = line_feed_index(&bytes[i..]) {
                    i += lf + 1;
                    reader.counter.add(lf as u32);
                    reader.counter.newline();
                    continue 'top;
                } else {
                    return more(reader, bytes, bytes.len() - i)
                }
            }
            // '`^@~
            return err(reader, format!("Can't parse a token starting with ({})", char::from(c)))
        }
        if whitespace(c) {
            println!("WS '{}' at {:?}", c as char, &reader.counter); // TODO
            if let Some(printing) = not_whitespace_index(&bytes[i..], &mut reader.counter) {
                println!("WS Some(printing) at {:?}", &reader.counter); // TODO
                i += printing;
                continue 'top;
            } else {
                return more(reader, bytes, 0)
            }
        }
        if c >= b'{' {
            if c == b'{' {
                use map::Map;
                let h = Map::new();
                println!("{{ at {:?}", &reader.counter); // TODO
                reader.pending.push(Pending::Map, h);
                reader.counter.add(1);
                i += 1;
                continue 'top;
            } else {
                let (p, u) = reader.pending.top();
                match p {
                    Pending::Map | Pending::Set => {
                        ready = u;
                        reader.pending.pop();
                        reader.counter.add(1);
                        i += 1;
                        break 'ready;
                    },
                    _ => {
                        return err(reader, format!("Bad closing brace."))
                    }
                }
            }
        }
        if c >= b'[' {
            if c == b'[' {
                use vector::Vector;
                let h = Vector::new();
                reader.pending.push(Pending::Vector, h);
                reader.counter.add(1);
                i += 1;
                continue 'top;
            } else {
                let (p, u) = reader.pending.top();
                if let Pending::Vector = p {
                    ready = u;
                    reader.pending.pop();
                    reader.counter.add(1);
                    i += 1;
                    break 'ready;
                } else {
                    return err(reader, format!("Bad closing bracket"))
                }
            }
        } else {
            use list::List;
            if c == b'(' {
                let h = List::new();
                reader.pending.push(Pending::List, h);
                reader.counter.add(1);
                i += 1;
                continue 'top;
            } else {
                let (p, u) = reader.pending.top();
                if let Pending::List = p {
                    ready = {
                        use transduce::Transducers;
                        let rev = u.handle().pour(Transducers::new(), List::new().handle());
                        u.handle().retire();
                        rev.unit()
                    };
                    reader.pending.pop();
                    reader.counter.add(1);
                    i += 1;
                    break 'ready;
                } else {
                    // TODO report expected collection (if any)
                    return err(reader, format!("Bad closing paren"))
                }
            }
        }
    } // ready
        'reready: loop {
            if reader.pending.is_empty() {
                return ReadResult::Ok { bytes_used: i as u32, value: ready };
            } else {
                match reader.pending.top_case() {
                    Pending::Tagged  => {
                        use tagged::Tagged;
                        let tag = Tagged::new(reader.pending.top_unit().handle(), ready.handle());
                        reader.pending.pop();
                        ready = tag;
                        continue 'reready;
                    },
                    Pending::Discard => {
                        ready.handle().retire();
                        reader.pending.pop_discard();
                    },
                    Pending::Map     => { reader.pending.push(Pending::Mapping, ready) },
                    Pending::Mapping => {
                        let (k, v) = (reader.pending.top_unit().handle(), ready.handle());
                        reader.pending.pop();
                        let n = reader.pending.top_unit().handle();
                        let (m, displaced) = n.assoc(k, v);
                        reader.pending.set_top(m.unit());
                        if !displaced.is_nil() {
                            let s = format!("Duplicate mapping to both {} and {}.", displaced, v);
                            displaced.retire();
                            return err(reader, s)
                        }
                    },
                    Pending::Namespace => {
                        reader.pending.top_unit().handle().retire();
                        reader.pending.pop();
                        continue 'reready;
                    },
                    _ => { // Vector List Set
                        let h = reader.pending.top_unit().handle().conj(ready.handle());
                        reader.pending.set_top(h.unit());
                    },
                }
                continue 'top;
            }
        } // reready
    } // top
}

// TODO
// maintaining counter position
// unicode in: string, symbol, char

pub fn prefix_map(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 2) >= bytes.len() {
        return more(reader, bytes, 2)
    }
    let e = bytes[i + 2];
    assert!(ascii(e));
    if !hit(e, NUM_NAME) {
        return err(reader, format!("Bad first character in prefix symbol (#:{}). \
                            Put a valid symbol right after the colon, like #:weather{{:high 58, :low 42}}.",
                                   char::from(e)))
    } else if let Some(prefix) = isolate_symbolic(&bytes[(i + 2)..]) {
        if !valid_name_start(prefix) {
            return err(reader, format!("Bad start of symbol after #: (#:{}). \
                                    Put a symbol (not a number, keyword etc) right after the colon, \
                                    like #:weather{{:high 58, :low 42}}.", from_utf8(prefix).unwrap()))
        } else if prefix.len() < 6 && (prefix == b"nil" || prefix == b"true" || prefix == b"false") {
            return err(reader, format!("Bad sequence. A map prefix (#:) \
                                       must be followed by a valid symbol (not true/false/nil)."))
        } else if slash_index(prefix).is_some() {
            return err(reader, format!("Bad symbol after #: (#:{}); \
                                        no slash (/) allowed. Instead, it should look like \
                                        #:weather{{:high 58, :low 42}}.", from_utf8(prefix).unwrap()))
        } else {
            let j = i + 2 /*#:*/ + prefix.len();
            reader.counter.add(2 + prefix.len() as u32);
            if let Some(printing) = not_whitespace_index(&bytes[j..], &mut reader.counter) {
                if bytes[j + printing] == b'{' {
                    use edn::reader::immediate_both;
                    reader.pending.push(Pending::Namespace, immediate_both(i + 2, prefix.len()));
                    use map::Map;
                    reader.pending.push(Pending::Map, Map::new());
                    let used = 2 /*#:*/ + prefix.len() + printing + 1 /*{*/;
                    return ReadResult::Ok { bytes_used: used as u32, value: Handle::NIL }
                } else {
                    return err(reader, format!("Bad sequence, a map prefix (#:{}) \
                                            followed by a ({}). The map prefix should be followed \
                                            by a map opening curly brace ({{), like \
                                            #:weather{{:high 58, :low 42}}.",
                                               from_utf8(prefix).unwrap(), char::from(bytes[j + printing])))
                }
            } else {
                return more(reader, bytes, bytes.len() - i)
            }
        }
    } else {
        return more(reader, bytes, bytes.len() - i)
    }
}

pub fn tagged(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if let Some(tag_sym) = isolate_symbolic(&bytes[(i + 1)..]) {
        if let Some(solidus) = slash_index(tag_sym) {
            if solidus == tag_sym.len() - 1 {
                return err(reader, format!("Bad tag symbol ({}). \
                                    Symbols cannot end with a slash.", from_utf8(tag_sym).unwrap()))
            }
            if !valid_name(&tag_sym[(solidus + 1)..]) {
                return err(reader, format!("Bad tag symbol ({}). \
                                    Name component (after /) is invalid for symbols.", from_utf8(tag_sym).unwrap()))
            }
            use symbol::Symbol;
            let h = Symbol::new(tag_sym, solidus as u32);
            reader.pending.push(Pending::Tagged, h);
            // reader.counter.add(tag_sym.len() as u32 + 1);
            return ReadResult::Ok { bytes_used: tag_sym.len() as u32 + 1, value: Handle::NIL }
        } else {
            if tag_sym.len() < 6 {
                if tag_sym == b"inst" {
                    if let Some(relative) = not_whitespace_index(&bytes[(i + 1 + 4)..], &mut reader.counter) {
                        let j = i + 1 + 4 + relative;
                        if let Some(close) = string_end_quote_index(&bytes[j..], &mut reader.counter) {
                            let inst_content = &bytes[(j + 1)..(j + close)];
                            use inst::Inst;
                            return match Inst::new_parsed(inst_content) {
                                Err(msg) => { err(reader, msg) },
                                Ok(h) => {
                                    // #inst "1980-11-14T07:22:41Z"
                                    // i     j                    j+close
                                    // 5 + relative + close + 1
                                    let bytes_used = (1 + 4 + relative + close + 1) as u32;
                                    ReadResult::Ok { bytes_used, value: h.unit() }
                                }
                            };
                        } else {
                            // Needmore
                            unimplemented!()
                        }
                    } else {
                        // NeedMore
                        unimplemented!()
                    }
                }
                if tag_sym == b"uuid" {
                    if let Some(relative) = not_whitespace_index(&bytes[(i + 1 + 4)..], &mut reader.counter) {
                        let j = i + 1 + 4 + relative;
                        if let Some(close) = string_end_quote_index(&bytes[j..], &mut reader.counter) {
                            let uuid_content = &bytes[(j + 1)..(j + close)];
                            use uuid::Uuid;
                            return match Uuid::new_parsed(uuid_content) {
                                Err(msg) => { err(reader, msg) },
                                Ok(h) => {
                                    let bytes_used = (1 + 4 + relative + close + 1) as u32;
                                    ReadResult::Ok { bytes_used, value: h.unit() }
                                }
                            };
                        } else {
                            // Needmore
                            unimplemented!()
                        }
                    } else {
                        // NeedMore
                        unimplemented!()
                    }
                }
                if tag_sym == b"nil" || tag_sym == b"true" || tag_sym == b"false" {
                    return err(reader, format!("Bad reader tag. Tag must be a valid symbol (not true/false/nil)."))
                }
            }
            use symbol::Symbol;
            let h = Symbol::new(tag_sym, 0);
            reader.pending.push(Pending::Tagged, h);
            // reader.counter.add(tag_sym.len() as u32 + 1);
            return ReadResult::Ok { bytes_used: tag_sym.len() as u32 + 1, value: Handle::NIL }
        }
    } else {
        return more(reader, bytes, bytes.len() - i)
    }
}

pub fn symbolic_numbers(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 2) >= bytes.len() {
        return more(reader, bytes, 2)
    }
    let e = bytes[i + 2];
    assert!(ascii(e));
    if hit(e, NUM_NAME) {
        if let Some(name) = isolate_symbolic(&bytes[(i + 2)..]) {
            use float_point::FloatPoint;
            let h = if name == b"Inf" { FloatPoint::inf() }
                else if name == b"-Inf" { FloatPoint::neg_inf() }
                    else if name == b"NaN" { FloatPoint::not_a_number() }
                        else {
                            return err(reader, format!("Invalid symbolic name (##{}). \
                                    Symbolic numbers are ##Inf ##-Inf or ##NaN.", from_utf8(name).unwrap()))
                        };
            return ReadResult::Ok { bytes_used: 2 + name.len() as u32, value: h.unit() }
        } else {
            return more(reader, bytes, bytes.len() - i)
        }
    } else {
        return err(reader, format!("Invalid character after double pound (##{}). \
                                   Symbolic numbers are ##Inf ##-Inf or ##NaN.", char::from(e)))
    }
}

pub fn character(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 1) >= bytes.len() {
        return more(reader, bytes, 1)
    }
    let d = bytes[i + 1];
    assert!(ascii(d));
    if !printing(d) {
        return err(reader, format!("Bad character escape; backslash cannot be followed by whitespace."))
    }
    if let Some(char_name) = isolate_symbolic(&bytes[(i + 1)..]) {
        use character::Character;
        if char_name.len() == 1 {
            return ReadResult::Ok { bytes_used: 2, value: Character::from_byte(d).unit() }
        } else {
            match parse_character(char_name) {
                Ok(h) => {
                    return ReadResult::Ok { bytes_used: char_name.len() as u32 + 1, value: h.unit() }
                },
                Err(msg) => {
                    return err(reader, msg)
                },
            }
        }
    } else {
        return more(reader, bytes, bytes.len() - i)
    }
}


pub const ALPHABET:  (u64, u64) = (0x_0000_0000_0000_0000, 0x_07FF_FFFE_07FF_FFFE); // azAZ
pub const BASE_10:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x_0000_0000_8000_0000); // 0123456789_
pub const ALPHANUM_: (u64, u64) = (0x_03FF_0000_0000_0000, 0x_07FF_FFFE_87FF_FFFE); // azAZ 0-9 _
pub const BASE_16:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x_0000_007E_8000_007E); // 0123456789_ afAF
pub const NUM_NAME:  (u64, u64) = (0x_F7FF_EC72_0000_0000, 0x_17FF_FFFE_87FF_FFFE); // alphanum .*+!-_?$%&=<>|:/
pub const NUM_NAME2: (u64, u64) = (0x_F7FF_ECFA_0000_0000, 0x_17FF_FFFE_87FF_FFFE); // NUM_NAME and #'
pub const SPECIALS:  (u64, u64) = (0x_0800_008C_0000_0000, 0x_4000_0001_5000_0001); // #;'"`^@~\
pub const DELIMITER: (u64, u64) = (0x_0800_1301_FFFF_FFFF, 0x_A800_0000_2800_0000); // WS, ()[]{} ; maybe "

pub fn digit(b: u8) -> bool {
    // b.wrapping_sub(b'0') < 10
    b <= b'9' && b >= b'0' // branch prediction
}

pub fn sign(b: u8) -> bool {
    b == b'+' || b == b'-'
}

pub fn ascii(b: u8) -> bool {
    (b & 0x80) == 0x00
}

pub fn printing(b: u8) -> bool {
    // see digit, single branch
    b'!' <= b && b <= b'~'
}

pub fn control_char(b: u8) -> bool {
    ascii(b) && !printing(b)
}

pub fn whitespace(b: u8) -> bool {
    // ascii(b) && hit(pattern) ?
    control_char(b) || b == b','
}

pub fn get_bit(x: u64, y: u64, idx: u8) -> u32 {
    let z = x ^ y;
    let word_idx = idx & 0x3F;
    let x_ = (x >> word_idx) as u32;
    let z_ = (z >> word_idx) as u32;
    let masked = z_ & (idx as u32 >> 6);
    (masked ^ x_) & 0x01
}

pub fn hit(b: u8, pattern: (u64, u64)) -> bool {
    get_bit(pattern.0, pattern.1, b) == 1
}

pub fn parse_character(s: &[u8]) -> Result<Handle, String> {
    use character::Character;
    if s[0] == b'u' {
        if s.len() != 5 {
            return Err(format!("Unrecognized character literal (\\{}). A unicode literal should have \
                                exactly four hex digits, like \\u03BB.", from_utf8(s).unwrap()))
        }
        if after_base16(&s[1..]).is_some() {
            return Err(format!("Bad unicode literal (\\{}). A unicode literal should have \
                                four hex digits (0-9 a-f A-F), like \\u03BB.", from_utf8(s).unwrap()))
        }
        return Ok(Character::from_four_hex(&s[1..]))
    }
    let mut b = b'_';
    loop {
        if s == b"newline" { b = b'\n'; break; }
        if s == b"return"  { b = b'\r'; break; }
        if s == b"space"   { b = b' ' ; break; }
        if s == b"tab"     { b = b'\t'; break; }
        return Err(format!("Unrecognized character name (\\{}). You can use \\newline \\space or \\tab. \
                            Or give a unicode literal in hex like \\u03BB.", from_utf8(s).unwrap()))
    }
    return Ok(Character::from_byte(b))
}

pub fn valid_name_start(s: &[u8]) -> bool {
    let a = s[0];
    if digit(a) || a == b':' || a == b'#' || a == b'\'' { // hit pattern instead?
        return false;
    }
    if s.len() == 1 { return true; }
    if digit(s[1]) && (sign(a) || a == b'.') {
        return false;
    }
    true
}

pub fn valid_name(s: &[u8]) -> bool {
    if !valid_name_start(s) {
        return false;
    }
    if s.len() > 1 && slash_index(s).is_some() {
        return false;
    }
    true
}

pub fn slash_index(s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if s[i] == b'/' {
            return Some(i);
        }
    }
    None
}

pub fn isolate_symbolic(s: &[u8]) -> Option<&[u8]> {
    for i in 1..s.len() {
        assert!(ascii(s[i]));
        if !hit(s[i], NUM_NAME2) {
            return Some(&s[0..i])
        }
    }
    None
}

pub fn first_not_in_set(char_set: (u64, u64), s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if !hit(s[i], char_set) {
            return Some(i);
        }
    }
    None
}

pub fn after_base16(s: &[u8]) -> Option<usize> {
    first_not_in_set(BASE_16, s)
}

pub fn line_feed_index(s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if s[i] == b'\n' {
            return Some(i);
        }
    }
    None
}

pub fn not_whitespace_index(s: &[u8], c: &mut Counter) -> Option<usize> {
    let mut cnt = *c;
    for i in 0..s.len() {
        let si = s[i];
        if !whitespace(si) {
            *c = cnt;
            return Some(i)
        }
        if si == b'\n' { cnt.newline() } else { cnt.add(1) }
    }
    None
}

pub fn string_end_quote_index(s: &[u8], c: &mut Counter) -> Option<usize> {
    let mut cnt = *c;
    // unroll groups of four
    // TODO check for \, skip next
    let mut i: usize = 1;
    for i in 1..s.len() {
        let si = s[i];
        if si == b'\n' { cnt.newline() } else { cnt.add(1) }
        if si == b'"' && s[i - 1] != b'\\' {
            *c = cnt;
            return Some(i)
        }
    }
    None
}

