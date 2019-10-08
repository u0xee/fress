// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use handle::Handle;

pub const BASE_10:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x_0000_0000_8000_0000); // 0123456789_
pub const ALPHANUM_: (u64, u64) = (0x_03FF_0000_0000_0000, 0x_07FF_FFFE_87FF_FFFE); // azAZ 0-9 _
pub const BASE_16:   (u64, u64) = (0x_03FF_0000_0000_0000, 0x_0000_007E_8000_007E); // 0123456789_ afAF

pub fn get_bit(x: u64, y: u64, idx: u8) -> u32 {
    let z = x ^ y;
    let word_idx = idx & 0x3F;
    let x_ = (x >> word_idx) as u32;
    let z_ = (z >> word_idx) as u32;
    let masked = z_ & (idx as u32 >> 6);
    (masked ^ x_) & 0x01
}

pub fn hit(b: u8, pattern: (u64, u64)) -> bool { get_bit(pattern.0, pattern.1, b) == 1 }

pub fn first_not_in_set(char_set: (u64, u64), s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        if !hit(s[i], char_set) {
            return Some(i);
        }
    }
    None
}

pub fn after_base10(s: &[u8]) -> Option<usize> { first_not_in_set(BASE_10, s) }
pub fn after_base16(s: &[u8]) -> Option<usize> { first_not_in_set(BASE_16, s) }

pub fn lowercase(c: u8) -> u8 {
    if c <= b'Z' && c >= b'A' {
        c + 32
    } else { c }
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
    if body[0] == b'0' && body.len() > 1 && hit(body[1], BASE_16) {
        return Err(format!("Numbers (other than 0) can't start with a 0 ({}).", from_utf8(s).unwrap()))
    }
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
                    if e + 1 == after_point.len() {
                        return Err(format!("Missing exponent in floating point number ({}).",
                                           from_utf8(s).unwrap()))
                    }
                    let (explicit_exp_sign, exp_negate) = parse_sign(after_point[e + 1]);
                    let exp = {
                        let start = e + 1 + if explicit_exp_sign { 1 } else { 0 };
                        &after_point[start..]
                    };
                    if exp.len() == 0 {
                        return Err(format!("Missing exponent digits in floating point number ({}).",
                                           from_utf8(s).unwrap()))
                    }
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
                use float_point::FloatPoint;
                return Ok(FloatPoint::parse(negate, whole, after_point, promote))
            }
        }
        if db == b'e' {
            if promote && int_or_float {
                return Err(format!("Floating point numbers can't end in N ({}). \
                                    Use M to indicate arbitrary precision for floats.",
                                   from_utf8(s).unwrap()))
            }
            if d + 1 == body.len() {
                return Err(format!("Missing exponent in floating point number ({}).",
                                   from_utf8(s).unwrap()))
            }
            let (explicit_exp_sign, exp_negate) = parse_sign(body[d + 1]);
            let exp = {
                let start = d + 1 + if explicit_exp_sign { 1 } else { 0 };
                &body[start..]
            };
            if exp.len() == 0 {
                return Err(format!("Missing exponent digits in floating point number ({}).",
                                   from_utf8(s).unwrap()))
            }
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
                return Err(format!("Bad digits before x in hex number ({}). \
                                    A hex number looks like 0x123ABC.",
                                   from_utf8(s).unwrap()))
            }
            let content = &body[2..];
            if content.len() == 0 {
                return Err(format!("Missing digits after x in hex number ({}).",
                                   from_utf8(s).unwrap()))
            }
            if after_base16(content).is_some() {
                return Err(format!("Bad hex number ({}), should have only hex digits (0-9 a-f A-F).",
                                   from_utf8(s).unwrap()))
            }
            if promote && !int_or_float {
                return Err(format!("Hex numbers can't end in M ({}). \
                                    Use N to indicate arbitrary precision for integrals.",
                                   from_utf8(s).unwrap()))
            }
            use integral::Integral;
            return Ok(Integral::parse_hex(negate, content, promote))
        }
        if db == b'r' {
            let radix = if d == 1 {
                body[0] - b'0'
            } else {
                if d > 2 || body[1] == b'_' {
                    return Err(format!("Bad digits before r in radix number ({}), \
                                        the radix must be 2-36. Like 2r101 or 36rABZ.",
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
            if content.len() == 0 {
                return Err(format!("Missing digits after r in radix number ({}).",
                                   from_utf8(s).unwrap()))
            }
            if let Some(p) = first_not_in_set(ALPHANUM_, content) {
                return Err(format!("Bad radix number ({}), character {} at position {} makes no sense.",
                                   from_utf8(s).unwrap(), char::from(content[p]), d + 1 + p + if explicit_sign { 1 } else { 0 }))
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
            if denom.len() == 0 {
                return Err(format!("Missing denominator in rational number ({}).",
                                   from_utf8(s).unwrap()))
            }
            if after_base10(denom).is_some() {
                return Err(format!("Bad denominator in rational number ({}).",
                                   from_utf8(s).unwrap()))
            }
            if promote {
                return Err(format!("Rational numbers can't contain an N or M ({}).",
                                   from_utf8(s).unwrap()))
            }
            use rational::Rational;
            return Ok(Rational::parse(negate, numer, denom))
        }
        return Err(format!("Bad number ({}), character {} at position {} makes no sense.",
                           from_utf8(s).unwrap(), char::from(db), d + if explicit_sign { 1 } else { 0 }))
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

