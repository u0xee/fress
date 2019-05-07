// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use handle::Handle;

use std::str::from_utf8;


// TODO design parsing machinery
// chew up whitespace, table contains bit for that
// gather digit runs, table contains bit for digit (maybe hex digit?)
// gather string runs, looking for:
//   - " (double-quote) end
//   - \ (backslash) escape
//   - (maybe special chars? DEL NULL etc?)
//
// parsing domains:
//  - whitespace (,)
//  - numbers (digits, . e E)
//  - strings (end of string, or escape)
//  - token delimiters (whitespace, [](){} ;)   >ASCII ?

// New parse context:
// chew up whitespace aka find token start, or delimiter (find
// dispatch based on first character:
//  is it a delimiter? start new collection
//  is it a string? collect string contents
//  is it an alphanumeric? collect a token, interpret it (symbol [nil true false], keyword, int, float)
//  backslash (char), hash (dispatch)

// spin through control characters, space and comma.
// string, dispatch char, aggregate controls, digit +-, comment, char, symbol/keyword, invalid
// simple tests: comment, string, char, dispatch char
// varied: aggregate controls, digit +-, symbol/kw

#[derive(Copy, Clone, Debug)]
pub enum Pending {
    Vector,
    List,
    Map,
    Mapping,
    Set,
    Tagged,
    Discard,
}

pub const STACK_SIZE: usize = 20;

pub struct PendingStack {
    pub count: usize,
    pub case: [Pending; STACK_SIZE],
    pub vals: [Handle; STACK_SIZE],
}

impl PendingStack {
    pub fn new() -> PendingStack {
        PendingStack { count: 0, case: [Pending::Vector; STACK_SIZE],
            vals: [Handle { unit:  Handle::NIL}; STACK_SIZE] }
    }
    pub fn empty(&self) -> bool {
        self.count == 0
    }
    pub fn push(&mut self, p: Pending, h: Handle) {
        self.case[self.count] = p;
        self.vals[self.count] = h;
        self.count += 1;
    }
    pub fn pop(&mut self) {
        self.count -= 1;
    }
    pub fn top_case(&self) -> Pending {
        self.case[self.count - 1]
    }
    pub fn top(&self) -> Handle {
        self.vals[self.count - 1]
    }
    pub fn set_top(&mut self, h: Handle) {
        self.vals[self.count - 1] = h;
    }
    pub fn any_discard(&self) -> bool {
        for i in 0..self.count {
            if let Pending::Discard = self.case[i] {
                return true;
            }
        }
        false
    }
}

pub enum Partial {
    Str(Handle),
    Comment,
}

#[derive(Copy, Clone, Debug)]
pub struct Counter {
    pub chr: u32,
    pub row: u32,
    pub col: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { chr: 0, row: 1, col: 1 }
    }
    pub fn newline(&mut self) {
        self.chr += 1;
        self.row += 1;
        self.col = 1;
    }
    pub fn add(&mut self, n: u32) {
        self.chr += n;
        self.col += n;
    }
}

pub struct EdnReader {
    pub partial: Option<Partial>,
    pub pending: PendingStack,
    pub discard: bool,
    pub counter: Counter,
}

#[derive(Debug)]
pub enum ReadResult {
    Ok       { bytes_used: u32, value: Value },
    NeedMore { bytes_not_used: u32 },
    Error    { location: Counter, message: String },
}

impl EdnReader {
    pub fn new() -> EdnReader {
        EdnReader { partial: None, pending: PendingStack::new(),
            discard: false, counter: Counter::new() }
    }

    pub fn read(&mut self, bytes: &[u8]) -> ReadResult {
        if self.partial.is_some() {
            unimplemented!()
        }
        read(self, bytes)
    }

    pub fn finish(&mut self) -> ReadResult {
        let res = {
            let res = self.read(b" ");
            match res {
                ReadResult::NeedMore { bytes_not_used: _ } => ReadResult::Error { location: self.counter, message: "".to_string() },
                _ => res,
            }
        };
        self.partial = None;
        // TODO free pending aggregates
        res
    }
}

pub fn last(a: &[u8], i: usize) -> bool {
    (i + 1) == a.len()
}


// profile guided optimization; slow path error branches.
pub fn read(reader: &mut EdnReader, bytes: &[u8]) -> ReadResult {
    let mut i = 0usize;
    let mut ready = Handle::nil();

    'top: loop { 'ready: loop {
        if i >= bytes.len() {
            return ReadResult::NeedMore { bytes_not_used: 0 };
        }
        let c = bytes[i];
        assert!(ascii(c));
        if hit(c, NUM_NAME) { // alphanum .*+!-_?$%&=<>|:/
            if let Some(sym) = isolate_symbolic(&bytes[i..]) {
                let res = if digit(c) || (sign(c) && sym.len() > 1 && digit(sym[1])) {
                    parse_numeric(sym)
                } else {
                    parse_symbol_keyword(sym)
                };
                match res {
                    Ok(h) => {
                        ready = h;
                        i += sym.len();
                        break 'ready;
                    },
                    Err(err_string) => {
                        return ReadResult::Error { location: reader.counter, message: err_string }
                    },
                }
            } else {
                return ReadResult::NeedMore { bytes_not_used: (bytes.len() - i) as u32 }
            }
        }
        if hit(c, SPECIALS) { // #,;'"`^@~\
            if c == b'#' {
                if (i + 1) >= bytes.len() {
                    return ReadResult::NeedMore { bytes_not_used: 1 };
                }
                let d = bytes[i + 1];
                assert!(ascii(d));

                if d == b'{' {
                    use set::Set;
                    let h = Set::new().handle();
                    reader.pending.push(Pending::Set, h);
                    i += 2;
                    continue 'top;
                }
                if d == b':' {
                    // read ns
                    // read map
                }
                if hit(d, ALPHABET) {
                    unimplemented!() // tagged elements
                }
                if d == b'_' {
                    reader.pending.push(Pending::Discard, Handle::nil());
                    reader.discard = true;
                    i += 2;
                    continue 'top;
                }
                if d == b'#' {
                    // Inf -Inf NaN
                    unimplemented!()
                }
                unimplemented!() // unknown dispatch character
            }
            if c == b'"' {
                if let Some(contents) = isolate_string_content(&bytes[i..]) {
                    // "hello\nWorld\"There\\Some"
                    // create new string, store in ready, and break 'ready
                    unimplemented!()
                } else {
                    // copy into new partial string and return NeedMore
                    unimplemented!()
                }
            }
            if c == b'\\' {
                // \a \C \3 \\ \space \tab \newline \uAFAF
                // get next byte, check s t n u, outside ascii
                unimplemented!()
            }
            if c == b',' {
                if let Some(printing) = not_whitespace_index(&bytes[i..]) {
                    i += printing;
                    reader.counter.add(printing as u32);
                    continue 'top;
                } else {
                    return ReadResult::NeedMore { bytes_not_used: 0 };
                }
            }
            if c == b';' {
                if let Some(lf) = line_feed_index(&bytes[i..]) {
                    i += lf + 1;
                    reader.counter.add(lf as u32);
                    reader.counter.newline();
                    continue 'top;
                } else {
                    reader.partial = Some(Partial::Comment);
                    reader.counter.add((bytes.len() - i) as u32);
                    return ReadResult::NeedMore { bytes_not_used: 0 }
                }
            }
            // '`^@~
            panic!("Can't parse {}.", c)
        }
        if control_char(c) {
            if let Some(printing) = not_whitespace_index(&bytes[i..]) {
                i += printing;
                reader.counter.add(printing as u32);
                continue 'top;
            } else {
                return ReadResult::NeedMore { bytes_not_used: 0 };
            }
        }
        if c >= b'{' {
            if c == b'{' {
                use map::Map;
                let h = Map::new().handle();
                reader.pending.push(Pending::Map, h);
                i += 1;
                continue 'top;
            } else {
                match reader.pending.top_case() {
                    Pending::Map | Pending::Set => {
                        ready = reader.pending.top();
                        reader.pending.pop();
                        i += 1;
                        break 'ready;
                    },
                    _ => {
                        return ReadResult::Error { location: reader.counter,
                            message: format!("Unmatched closing brace {:?}", reader.pending.top_case()).to_string() }
                    }
                }
            }
        }
        if c >= b'[' {
            if c == b'[' {
                use vector::Vector;
                let h = Vector::new().handle();
                reader.pending.push(Pending::Vector, h);
                i += 1;
                continue 'top;
            } else {
                if let Pending::Vector = reader.pending.top_case() {
                    ready = reader.pending.top();
                    reader.pending.pop();
                    i += 1;
                    break 'ready;
                } else {
                    return ReadResult::Error { location: reader.counter,
                        message: format!("Unmatched closing bracket").to_string() }
                }
            }
        } else {
            if c == b'(' {
                use list::List;
                let h = List::new().handle();
                reader.pending.push(Pending::List, h);
                i += 1;
                continue 'top;
            } else {
                if let Pending::List = reader.pending.top_case() {
                    ready = reader.pending.top();
                    use transduce::Transducers;
                    use list::List;
                    ready = ready.pour(Transducers::new(), List::new().handle());
                    reader.pending.pop();
                    i += 1;
                    break 'ready;
                } else {
                    return ReadResult::Error { location: reader.counter,
                        message: format!("Unmatched closing paren").to_string() }
                }
            }
        }
    } // ready
        if reader.pending.empty() {
            return ReadResult::Ok { bytes_used: i as u32, value: ready.value() };
        } else {
            match reader.pending.top_case() {
                Pending::Tagged  => { unimplemented!() },
                Pending::Discard => {
                    ready.retire();
                    reader.pending.pop();
                    reader.discard = reader.pending.any_discard();
                },
                Pending::Map     => { reader.pending.push(Pending::Mapping, ready) },
                Pending::Mapping => {
                    let (k, v) = (reader.pending.top(), ready);
                    reader.pending.pop();
                    let (m, displaced) = reader.pending.top().assoc(k, v);
                    if !displaced.is_nil() {
                        return ReadResult::Error { location: reader.counter,
                            message: format!("Duplicate key in map").to_string()};
                    }
                    reader.pending.set_top(m);
                },
                _ => { // Vector List Set
                    let h = reader.pending.top().conj(ready);
                    reader.pending.set_top(h);
                },
            }
            continue 'top;
        }
    } // top
    unimplemented!()
}


pub const ALPHABET:  (u64, u64) = (0x_0000_0000_0000_0000, 0x07FF_FFFE_07FF_FFFE); // azAZ
pub const BASE_10:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x0000_0000_8000_0000); // 0123456789_
pub const ALPHANUM_: (u64, u64) = (0x_03FF_0000_0000_0000, 0x07FF_FFFE_87FF_FFFE); // azAZ 0-9 _
pub const BASE_16:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x0000_007E_8000_007E); // 0123456789_ afAF
pub const NUM_NAME:  (u64, u64) = (0x_F7FF_EC72_0000_0000, 0x17FF_FFFE_87FF_FFFE); // alphanum .*+!-_?$%&=<>|:/
pub const NUM_NAME2: (u64, u64) = (0x_F7FF_ECFA_0000_0000, 0x17FF_FFFE_87FF_FFFE); // NUM_NAME and #'
pub const SPECIALS:  (u64, u64) = (0x_0800_108C_0000_0000, 0x4000_0001_5000_0001); // #,;'"`^@~\
pub const DELIMITER: (u64, u64) = (0x_0800_1301_FFFF_FFFF, 0xA800_0000_2800_0000); // WS, ()[]{} ; maybe "

pub fn digit(b: u8) -> bool {
    b <= b'9' && b >= b'0' // branch prediction
}

pub fn sign(b: u8) -> bool {
    b == b'+' || b == b'-'
}

pub fn ascii(b: u8) -> bool {
    (b & 0x80) == 0x00
}

pub fn control_char(b: u8) -> bool {
    ascii(b) && !(b'!' <= b && b <= b'~')
}

pub fn whitespace(b: u8) -> bool {
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

pub fn parse_sign(c: u8) -> (bool, bool) {
    let (explicit_sign, negate) = match c {
        b'-' => (true, true),
        b'+' => (true, false),
        _    => (false, false),
    };
    (explicit_sign, negate)
}

pub fn parse_numeric(s: &[u8]) -> Result<Handle, String> {
    let (explicit_sign, negate) = parse_sign(s[0]);
    let (promote, int_or_float) = match s[s.len() - 1] {
        b'N' => (true, true),
        b'M' => (true, false),
        _    => (false, false),
    };
    let body = {
        let start = if explicit_sign { 1 } else { 0 };
        let end = s.len() - if promote { 1 } else { 0 };
        &s[start..end]
    };
    if let Some(d) = after_base10(body) {
        let db = lowercase(body[d]);
        if db == b'.' {
            if promote && int_or_float {
                return Err(format!("Floating point numbers can't end in N ({}). \
                                    Use M to indicate arbitrary precision for floats.",
                                   from_utf8(s).unwrap()))
            }
            let whole = &body[..d];
            let after_point = &body[(d + 1)..];
            if let Some(e) = after_base10(after_point) {
                let eb = lowercase(after_point[e]);
                if eb == b'e' {
                    let (explicit_exp_sign, exp_negate) = parse_sign(after_point[e + 1]);
                    let exp = {
                        let start = e + 1 + if explicit_exp_sign { 1 } else { 0 };
                        &after_point[start..]
                    };
                    if after_base10(exp).is_some() {
                        return Err(format!("Bad exponent in floating point number ({}).",
                                           from_utf8(s).unwrap()))
                    }
                    let part = &after_point[..e];
                    use float_point::FloatPoint;
                    return Ok(FloatPoint::parse_exp(negate, whole, part, exp_negate, exp, promote))
                }
                return Err(format!("Bad fractional part in floating point number ({}).",
                                   from_utf8(s).unwrap()))
            } else {
                // only digits left
                let part = after_point;
                use float_point::FloatPoint;
                return Ok(FloatPoint::parse(negate, whole, part, promote))
            }
        }
        if db == b'e' {
            if promote && int_or_float {
                return Err(format!("Floating point numbers can't end in N ({}). \
                                    Use M to indicate arbitrary precision for floats.",
                                   from_utf8(s).unwrap()))
            }
            let (explicit_exp_sign, exp_negate) = parse_sign(body[d + 1]);
            let exp = {
                let start = d + 1 + if explicit_exp_sign { 1 } else { 0 };
                &body[start..]
            };
            if after_base10(exp).is_some() {
                return Err(format!("Bad exponent in floating point number ({}).",
                                   from_utf8(s).unwrap()))
            }
            let whole = &body[..d];
            let part = &b""[..];
            use float_point::FloatPoint;
            return Ok(FloatPoint::parse_exp(negate, whole, part, exp_negate, exp, promote))
        }
        if db == b'x' {
            if d != 1 || body[0] != b'0' {
                return Err(format!("Bad number ({}). A hex number looks like 0x123ABC.",
                                   from_utf8(s).unwrap()))
            }
            if promote && !int_or_float {
                return Err(format!("Hex numbers can't end in M ({}). \
                                    Use N to indicate arbitrary precision for integrals.",
                                   from_utf8(s).unwrap()))
            }
            let content = &body[2..];
            if after_base16(content).is_some() {
                return Err(format!("Bad hex number ({}), contains non-hex characters.",
                                   from_utf8(s).unwrap()))
            }
            use integral::Integral;
            return Ok(Integral::parse_hex(negate, content, promote))
        }
        if db == b'r' {
            if d > 2 {
                return Err(format!("Bad number ({}), the radix must be 2-36. Like 36rABC.",
                                   from_utf8(s).unwrap()))
            }
            let radix = if d == 1 {
                body[0] - b'0'
            } else {
                if !digit(body[1]) {
                    return Err(format!("Bad number ({}), the radix must be 2-36. Like 36rABC.",
                                       from_utf8(s).unwrap()))
                }
                (body[0] - b'0') * 10 + (body[1] - b'0')
            };
            if radix < 2 || radix > 36 {
                return Err(format!("Bad number ({}), the radix must be 2-36. Like 36rABC.",
                                   from_utf8(s).unwrap()))
            }
            let content = {
                let start = d + 1 + if explicit_sign { 1 } else { 0 };
                &s[start..]
            };
            if first_not_in_set(ALPHANUM_, content).is_some() {
                return Err(format!("Bad number ({}), digits should be 0-9, a-z or A-Z.",
                                   from_utf8(s).unwrap()))
            }
            use integral::Integral;
            if let Some(h) = Integral::parse_radix(negate, radix as u32, content) {
                return Ok(h)
            } else {
                return Err(format!("Bad number ({}), digits should be valid for radix {}.",
                                   from_utf8(s).unwrap(), radix))
            }
        }
        if db == b'/' {
            let numer = &body[..d];
            let denom = &body[(d + 1)..];
            if after_base10(denom).is_some() {
                return Err(format!("Bad denominator in rational number ({}).",
                                   from_utf8(s).unwrap()))
            }
            use rational::Rational;
            return Ok(Rational::parse(negate, numer, denom))
        }
        return Err(format!("Bad number ({}), character {} at position {} makes no sense.",
                           from_utf8(s).unwrap(), char::from(db), d))
    } else { // plain base 10 number
        use integral::Integral;
        if promote {
            if int_or_float {
                return Ok(Integral::parse(negate, body, true))
            } else {
                use float_point::FloatPoint;
                let part = &b""[..];
                return Ok(FloatPoint::parse(negate, body, part, true))
            }
        } else {
            return Ok(Integral::parse(negate, body, false))
        }
    }
}

pub fn lowercase(c: u8) -> u8 {
    if c <= b'Z' && c >= b'A' {
        c + 32
    } else { c }
}

pub fn parse_symbol_keyword(s: &[u8]) -> Result<Handle, String> {
    let solidus = if let Some(solidus) = prefix_slash(s) {
        let prefix_len = solidus - if s[0] == b':' { 1 } else { 0 };
        let name_len = s.len() - solidus - 1;
        if prefix_len == 0 || name_len == 0 {
            return Err(format!("Can't have a symbol/keyword with an empty prefix or name ({})",
                               from_utf8(s).unwrap()))
        }
        if !valid_name(&s[(solidus + 1)..]) {
            return Err(format!("Name component of symbol/keyword is invalid ({})",
                               from_utf8(s).unwrap()))
        }
        solidus
    } else { 0 };
    if s[0] == b':' {
        if !valid_name_start(&s[1..]) {
            return Err(format!("Keyword has invalid starting characters ({})",
                               from_utf8(s).unwrap()))
        }
        use keyword::Keyword;
        return Ok(Keyword::new(s, solidus as u32).handle())
    } else {
        if s.len() < 6 {
            if s == b"nil"   { return Ok(Handle::nil()) }
            if s == b"true"  { return Ok(Handle::tru()) }
            if s == b"false" { return Ok(Handle::fals()) }
        }
        if s[0] == b'.' && s.len() > 1 && digit(s[1]) {
            return Err(format!("Not a valid token ({})",
                               from_utf8(s).unwrap()))
        }
        use symbol::Symbol;
        return Ok(Symbol::new(s, solidus as u32).handle())
    }
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
    if slash_index(s).is_some() {
        return false;
    }
    true
}

pub fn first_not_in_set(char_set: (u64, u64), s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if !hit(s[i], char_set) {
            return Some(i);
        }
    }
    None
}

pub fn after_base10(s: &[u8]) -> Option<usize> {
    first_not_in_set(BASE_10, s)
}

pub fn after_base16(s: &[u8]) -> Option<usize> {
    first_not_in_set(BASE_16, s)
}

pub fn prefix_slash(s: &[u8]) -> Option<usize> {
    if s.len() == 1 {
        return None;
    } else {
        slash_index(s)
    }
}

pub fn isolate_symbolic(s: &[u8]) -> Option<&[u8]> {
    for i in 1..s.len() {
        if !hit(s[i], NUM_NAME2) {
            return Some(&s[0..i])
        }
    }
    None
}

pub fn slash_index(s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if s[i] == b'/' {
            return Some(i);
        }
    }
    None
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
            return Some(i);
        }
    }
    None
}

pub fn isolate_string_content(s: &[u8]) -> Option<&[u8]> {
    // unroll groups of four
    for i in 1..s.len() {
        if s[i] == b'"' && s[i - 1] != b'\\' {
            return Some(&s[1..i])
        }
    }
    None
}

