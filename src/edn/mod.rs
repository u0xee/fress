// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use handle::Handle;

pub mod number;
pub mod name;
pub mod reader;
use self::reader::{EdnReader, ReadResult, Pending, Counter};


pub fn err(reader: &mut EdnReader, msg: String) -> ReadResult {
    let res = ReadResult::Error { location: reader.counter, message: msg };
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
        if hit(c, NUM_NAME) || !ascii(c) { // alphanum .*+!-_?$%&=<>|:/
            let sym = match isolate_symbolic(&bytes[i..]) {
                Some(s) => s,
                None => { return more(reader, bytes, bytes.len() - i) },
            };
            let str_sym = match from_utf8(sym) {
                Ok(s) => s,
                Err(e) => {
                    return err(reader, format!("Invalid utf-8 in symbolic contents."))
                }
            };
            let res = if digit(c) || (sign(c) && sym.len() > 1 && digit(sym[1])) {
                number::parse_numeric(sym)
            } else {
                use edn::reader::reference;
                let default = reader.pending.default_ns().map(|ns| reference(ns, bytes));
                name::parse_symbol_keyword(sym, default)
            };
            match res {
                Ok(h) => {
                    ready = h.unit();
                    reader.counter = reader.counter.count(str_sym);
                    i += sym.len();
                    break 'ready;
                },
                Err(err_string) => {
                    return err(reader, err_string)
                },
            }
        }
        if hit(c, SPECIALS) { // #,;'"`^@~\
            if c == b'#' {
                if (i + 1) >= bytes.len() {
                    return more(reader, bytes, 1)
                }
                let d = bytes[i + 1];
                if d == b'{' {
                    use set::Set;
                    let h = Set::new();
                    reader.pending.push(Pending::Set, h);
                    reader.counter = reader.counter.add_ascii(2);
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
                if hit(d, ALPHABET) || !ascii(d) {
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
                    reader.counter = reader.counter.add_ascii(2);
                    i += 2;
                    continue 'top;
                }
                if d == b'#' {
                    match symbolic_numbers(reader, bytes, i) {
                        ReadResult::Ok { bytes_used, value } => {
                            ready = value;
                            reader.counter = reader.counter.add_ascii(bytes_used);
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
                let quote_index = match string_end_quote_index(str_start) {
                    Some(q) => q,
                    None => { return more(reader, bytes, bytes.len() - i) },
                };
                let contents = &str_start[1..quote_index];
                let str_contents = match from_utf8(contents) {
                    Ok(x) => { x },
                    Err(e) => {
                        return err(reader, format!("Invalid utf-8 in string contents."))
                    },
                };
                use string::Str;
                match Str::new_escaping(contents) {
                    Ok(h) => {
                        reader.counter = reader.counter.add_ascii(1)
                            .count(str_contents).add_ascii(1);
                        ready = h.unit();
                        i += quote_index + 1 /*end quote*/;
                        break 'ready;
                    },
                    Err(msg) => {
                        return err(reader, msg)
                    },
                }
            }
            if c == b'\\' {
                match character(reader, bytes, i) {
                    ReadResult::Ok { bytes_used, value } => {
                        ready = value;
                        i += bytes_used as usize;
                        break 'ready;
                    },
                    res => { return res },
                }
            }
            if c == b';' {
                let lf = match line_feed_index(&bytes[i..]) {
                    Some(lf) => lf,
                    None => { return more(reader, bytes, bytes.len() - i) },
                };
                i += lf + 1;
                reader.counter = reader.counter.add_ascii(lf as u32).newline();
                continue 'top;
            }
            // '`^@~
            return err(reader, format!("Can't parse a token starting with ({})", char::from(c)))
        }
        if whitespace(c) {
            let ws = match not_whitespace_index(&bytes[i..]) {
                Some(printing) => &bytes[i..(i + printing)],
                None => &bytes[i..],
            };
            reader.counter = reader.counter.count_ascii(ws);
            i += ws.len();
            continue 'top;
        }
        if c >= b'{' {
            if c == b'{' {
                use map::Map;
                let h = Map::new();
                reader.pending.push(Pending::Map, h);
                reader.counter = reader.counter.add_ascii(1);
                i += 1;
                continue 'top;
            }
            if reader.pending.is_empty() {
                return err(reader, format!("Unexpected closing brace }} \
                    not inside a map or set."))
            }
            let (p, u) = reader.pending.top();
            match p {
                Pending::Map | Pending::Set => {
                    ready = u;
                    reader.pending.pop();
                    reader.counter = reader.counter.add_ascii(1);
                    i += 1;
                    break 'ready;
                },
                _ => {
                    return err(reader, format!("Unexpected closing brace }} \
                        inside a {}.", p.name()))
                }
            }
        }
        if c >= b'[' {
            if c == b'[' {
                use vector::Vector;
                let h = Vector::new();
                reader.pending.push(Pending::Vector, h);
                reader.counter = reader.counter.add_ascii(1);
                i += 1;
                continue 'top;
            }
            if reader.pending.is_empty() {
                return err(reader, format!("Unexpected closing bracket ] \
                    not inside a vector."))
            }
            let (p, u) = reader.pending.top();
            if let Pending::Vector = p {
                ready = u;
                reader.pending.pop();
                reader.counter = reader.counter.add_ascii(1);
                i += 1;
                break 'ready;
            } else {
                return err(reader, format!("Unexpected closing bracket ] \
                    inside a {}.", p.name()))
            }
        } else {
            use list::List;
            if c == b'(' {
                let h = List::new();
                reader.pending.push(Pending::List, h);
                reader.counter = reader.counter.add_ascii(1);
                i += 1;
                continue 'top;
            }
            if reader.pending.is_empty() {
                return err(reader, format!("Unexpected closing paren ) \
                    not inside a list."))
            }
            let (p, u) = reader.pending.top();
            if let Pending::List = p {
                ready = {
                    use transduce::Transducers;
                    let rev = u.handle().pour(Transducers::new(), List::new().handle());
                    u.handle().retire();
                    rev.unit()
                };
                reader.pending.pop();
                reader.counter = reader.counter.add_ascii(1);
                i += 1;
                break 'ready;
            } else {
                return err(reader, format!("Unexpected closing paren ) \
                    inside a {}.", p.name()))
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

pub fn prefix_map(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 2) >= bytes.len() {
        return more(reader, bytes, 2)
    }
    let e = bytes[i + 2];
    if !hit(e, NUM_NAME) && ascii(e) {
        return err(reader, format!("Bad first character in prefix symbol (#:{}). \
                            Put a valid symbol right after the colon, like #:weather{{:high 58, :low 42}}.",
                                   char::from(e)))
    }
    let prefix = match isolate_symbolic(&bytes[(i + 2)..]) {
        Some(s) => s,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let str_prefix = match from_utf8(prefix) {
        Ok(s) => s,
        Err(e) => {
            return err(reader, format!("Invalid utf-8 in map prefix symbol contents."))
        }
    };
    if !valid_name_start(prefix) {
        return err(reader, format!("Bad start of symbol after #: (#:{}). \
                                    Put a symbol (not a number, keyword etc) right after the colon, \
                                    like #:weather{{:high 58, :low 42}}.", str_prefix))
    }
    if prefix.len() < 6 && (prefix == b"nil" || prefix == b"true" || prefix == b"false") {
        return err(reader, format!("Bad sequence. A map prefix (#:) \
                                       must be followed by a valid symbol (not true/false/nil)."))
    }
    if slash_index(prefix).is_some() {
        return err(reader, format!("Bad symbol after #: (#:{}); \
                                        no slash (/) allowed. Instead, it should look like \
                                        #:weather{{:high 58, :low 42}}.", str_prefix))
    }
    let after_prefix = &bytes[(i + 2 + prefix.len())..];
    let printing = match not_whitespace_index(after_prefix) {
        Some(p) => p,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let ws = &after_prefix[..printing];
    if after_prefix[printing] != b'{' {
        return err(reader, format!("Bad sequence, a map prefix (#:{}) \
                           followed by a {}. The map prefix should be followed \
                           by a map opening curly brace {{, like \
                           #:weather{{:high 58, :low 42}}.",
                           str_prefix, char::from(after_prefix[printing])))
    }
    use edn::reader::immediate_both;
    reader.pending.push(Pending::Namespace, immediate_both(i + 2 /*#:*/, prefix.len()));
    use map::Map;
    reader.pending.push(Pending::Map, Map::new());
    let bytes_used = (2 /*#:*/ + prefix.len() + ws.len() + 1 /*{*/) as u32;
    let ctr = reader.counter.add_ascii(2).count(str_prefix).count_ascii(ws).add_ascii(1);
    reader.counter = ctr;
    return ReadResult::Ok { bytes_used, value: Handle::NIL }
}

pub fn tagged(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    let tag_sym = match isolate_symbolic(&bytes[(i + 1)..]) {
        Some(s) => s,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let str_tag = match from_utf8(tag_sym) {
        Ok(s) => s,
        Err(e) => {
            return err(reader, format!("Invalid utf-8 in tag symbol."))
        }
    };
    if let Some(solidus) = slash_index(tag_sym) {
        if solidus == tag_sym.len() - 1 {
            return err(reader, format!("Bad tag symbol ({}). \
                                    Symbols cannot end with a slash.", str_tag))
        }
        if !valid_name(&tag_sym[(solidus + 1)..]) {
            return err(reader, format!("Bad tag symbol ({}). \
                                    Name component (after /) is invalid for symbols.", str_tag))
        }
        use symbol::Symbol;
        let h = Symbol::new(tag_sym, solidus as u32);
        reader.pending.push(Pending::Tagged, h);
        reader.counter = reader.counter.add_ascii(1).count(str_tag);
        return ReadResult::Ok { bytes_used: tag_sym.len() as u32 + 1, value: Handle::NIL }
    }
    if tag_sym.len() < 6 {
        if tag_sym == b"inst" { return tagged_inst(reader, bytes, i) }
        if tag_sym == b"uuid" { return tagged_uuid(reader, bytes, i) }
        if tag_sym == b"nil" || tag_sym == b"true" || tag_sym == b"false" {
            return err(reader, format!("Bad reader tag. \
                    Tag must be a valid symbol (not true/false/nil)."))
        }
    }
    use symbol::Symbol;
    let h = Symbol::new(tag_sym, 0);
    reader.pending.push(Pending::Tagged, h);
    reader.counter = reader.counter.add_ascii(1).count(str_tag);
    return ReadResult::Ok { bytes_used: tag_sym.len() as u32 + 1, value: Handle::NIL }
}

pub fn tagged_inst(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    let after_tag = &bytes[(i + 5 /*#inst*/)..];
    let printing = match not_whitespace_index(after_tag) {
        Some(p) => p,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let ws = &after_tag[..printing];
    let form = &after_tag[printing..];
    if form[0] != b'"' {
        return err(reader, format!("Bad inst. The content should be \
                                      a string, like: #inst \"1980-11-14T07:22:41Z\"."))
    }
    let end_quote = match string_end_quote_index(form) {
        Some(e) => e,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let content = &form[1..end_quote];
    let str_content = match from_utf8(content) {
        Ok(s) => s,
        Err(e) => {
            return err(reader, format!("Invalid utf-8 in inst contents."))
        }
    };
    use inst::Inst;
    return match Inst::new_parsed(content) {
        Err(msg) => { err(reader, msg) },
        Ok(h) => {
            let ctr = reader.counter.add_ascii(5 /*#inst*/)
                .count_ascii(ws).add_ascii(1)
                .count(str_content).add_ascii(1);
            reader.counter = ctr;
            let bytes_used = (5 + ws.len() + 1 + content.len() + 1) as u32;
            ReadResult::Ok { bytes_used, value: h.unit() }
        }
    };
}

pub fn tagged_uuid(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    let after_tag = &bytes[(i + 5 /*#uuid*/)..];
    let printing = match not_whitespace_index(after_tag) {
        Some(p) => p,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let ws = &after_tag[..printing];
    let form = &after_tag[printing..];
    if form[0] != b'"' {
        return err(reader, format!("Bad uuid. The content should be \
                             a string, like: #uuid \"F81D4FAE-7DEC-11D0-A765-00A0C91E6BF6\"."))
    }
    let end_quote = match string_end_quote_index(form) {
        Some(e) => e,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    let content = &form[1..end_quote];
    let str_content = match from_utf8(content) {
        Ok(s) => s,
        Err(e) => {
            return err(reader, format!("Invalid utf-8 in uuid contents."))
        }
    };
    use uuid::Uuid;
    return match Uuid::new_parsed(content) {
        Err(msg) => { err(reader, msg) },
        Ok(h) => {
            let ctr = reader.counter.add_ascii(5 /*#uuid*/)
                .count_ascii(ws).add_ascii(1)
                .count(str_content).add_ascii(1);
            reader.counter = ctr;
            let bytes_used = (5 + ws.len() + 1 + content.len() + 1) as u32;
            ReadResult::Ok { bytes_used, value: h.unit() }
        }
    };
}

pub fn symbolic_numbers(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 2) >= bytes.len() {
        return more(reader, bytes, 2)
    }
    let e = bytes[i + 2];
    if !hit(e, NUM_NAME) && ascii(e) {
        return err(reader, format!("Invalid character after double pound (##{}). \
                                   Symbolic numbers are ##Inf ##-Inf or ##NaN.", char::from(e)))
    }
    let name = match isolate_symbolic(&bytes[(i + 2)..]) {
        Some(n) => n,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    use float_point::FloatPoint;
    let h = if name == b"Inf" { FloatPoint::inf() }
    else if name == b"-Inf" { FloatPoint::neg_inf() }
    else if name == b"NaN" { FloatPoint::not_a_number() }
    else {
        return err(reader, format!("Invalid symbolic name (##{}). \
                                    Symbolic numbers are ##Inf ##-Inf or ##NaN.", from_utf8(name).unwrap()))
    };
    return ReadResult::Ok { bytes_used: 2 + name.len() as u32, value: h.unit() }
}

pub fn character(reader: &mut EdnReader, bytes: &[u8], i: usize) -> ReadResult {
    if (i + 1) >= bytes.len() {
        return more(reader, bytes, 1)
    }
    let d = bytes[i + 1];
    if !printing(d) && ascii(d) {
        return err(reader, format!("Bad character escape; backslash cannot be followed by whitespace."))
    }
    let char_name = match isolate_symbolic(&bytes[(i + 1)..]) {
        Some(n) => n,
        None => { return more(reader, bytes, bytes.len() - i) },
    };
    use character::Character;
    if char_name.len() == 1 {
        reader.counter = reader.counter.add_ascii(2);
        return ReadResult::Ok { bytes_used: 2, value: Character::from_byte(d).unit() }
    }
    let str_char_name = match from_utf8(char_name) {
        Ok(s) => s,
        Err(e) => {
            return err(reader, format!("Invalid utf-8 in character literal contents."))
        }
    };
    match parse_character(char_name) {
        Ok(h) => {
            let bytes_used = char_name.len() as u32 + 1;
            reader.counter = reader.counter.add_ascii(1).count(str_char_name);
            return ReadResult::Ok { bytes_used, value: h.unit() }
        },
        Err(msg) => {
            return err(reader, msg)
        },
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

pub fn unicode_start(b: u8) -> bool {
    (b & 0xC0) == 0xC0
}

pub fn unicode_cont(b: u8) -> bool {
    (b & 0xC0) == 0x80
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
    let word_idx = (idx & 0x3F) as u64;
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
    let c = s[0];
    if c == b'u' {
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
    if !ascii(c) {
        if unicode_cont(c) {
            return Err(format!("Bad utf-8 continuation byte 0x{:X}, \
            not inside a multibyte code point.", c))
        }
        let leading_ones = if c & 0xE0 == 0xC0 { 2 }
        else if c & 0xF0 == 0xE0 { 3 }
        else if c & 0xF8 == 0xF0 { 4 }
        else {
            return Err(format!("Bad utf-8 byte 0x{:X}.", c))
        };
        if s.len() < leading_ones {
            return Err(format!("Bad utf-8, byte 0x{:X} \
            should be followed by {} continuation bytes.", c, leading_ones))
        }
        for i in 1..leading_ones {
            if !unicode_cont(s[i]) {
                return Err(format!("Bad utf-8, byte 0x{:X} \
                should be followed by continuation bytes, not 0x{:X}.", c, s[i]))
            }
        }
        if s.len() > leading_ones {
            return Err(format!("Bad character literal (\\{}) more than one character.", from_utf8(s).unwrap()))
        }
        let u = from_utf8(s).unwrap().chars().next().unwrap();
        return Ok(Character::new(u))
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
        let b = s[i];
        if !hit(b, NUM_NAME2) && ascii(b) {
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

pub fn not_whitespace_index(s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if !whitespace(s[i]) {
            return Some(i)
        }
    }
    None
}

pub fn string_end_quote_index(s: &[u8]) -> Option<usize> {
    let mut i = 1usize;
    let len = s.len();
    loop {
        if i < len {
            let b = s[i];
            if b == b'"' { return Some(i) }
            i += if b == b'\\' { 2 } else { 1 };
            continue;
        } else {
            break;
        }
    }
    None
}


