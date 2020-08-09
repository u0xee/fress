// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::str::from_utf8;
use handle::Handle;

pub fn digit(b: u8) -> bool {
    // b.wrapping_sub(b'0') < 10
    b <= b'9' && b >= b'0' // branch prediction
}

pub fn sign(b: u8) -> bool { b == b'+' || b == b'-' }

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

pub fn prefix_slash(s: &[u8]) -> Option<usize> {
    if s.len() == 1 {
        return None;
    } else {
        slash_index(s)
    }
}

pub fn parse_symbol_keyword(s: &[u8], default: Option<&[u8]>) -> Result<Handle, String> {
    let solidus = if let Some(solidus) = prefix_slash(s) {
        let prefix_len = solidus - if s[0] == b':' { 1 } else { 0 };
        let name_len = s.len() - solidus - 1;
        if prefix_len == 0 || name_len == 0 {
            return Err(format!("Can't have a symbol/keyword with an empty prefix or name ({})",
                               from_utf8(s).unwrap()))
        }
        if !valid_name(&s[(solidus + 1)..]) {
            return Err(format!("Name component (after /) of symbol/keyword is invalid ({})",
                               from_utf8(s).unwrap()))
        }
        solidus
    } else { 0 };
    if s[0] == b':' {
        log!("parsing keyword {}", from_utf8(s).unwrap());
        if !valid_name_start(&s[1..]) {
            return Err(format!("Keyword has invalid starting characters ({})",
                               from_utf8(s).unwrap()))
        }
        use keyword;
        if let Some(d) = default {
            if solidus == 0 {
                return Ok(keyword::new_prefix_name(d, &s[1..]).handle())
            }
            if solidus == 2 && s[1] == b'_' {
                return Ok(keyword::new_from_name(&s[3..]).handle())
            }
        }
        return Ok(keyword::new(s, solidus as u32).handle())
    } else {
        log!("parsing symbol {}", from_utf8(s).unwrap());
        if s.len() < 6 {
            if s == b"nil"   { return Ok(Handle::nil()) }
            if s == b"true"  { return Ok(Handle::tru()) }
            if s == b"false" { return Ok(Handle::fals()) }
        }
        if s.len() > 1 {
            let after_sign = if sign(s[0]) { 1 } else { 0 };
            if s[after_sign] == b'.' && after_sign + 1 < s.len() && digit(s[after_sign + 1]) {
                return Err(format!("Not a valid token ({}). Floating point numbers must \
                                have a digit before the point, like 0.1 or +1.42.",
                                   from_utf8(s).unwrap()))
            }
        }
        use symbol;
        if let Some(d) = default {
            if solidus == 0 {
                return Ok(symbol::new_prefix_name(d, s).handle())
            }
            if solidus == 1 && s[0] == b'_' {
                return Ok(symbol::new(&s[2..], 0).handle())
            }
        }
        return Ok(symbol::new(s, solidus as u32).handle())
    }
}
