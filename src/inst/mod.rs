// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//use std::str::from_utf8;
use std::fmt;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use handle::Handle;

pub mod guide;
use self::guide::{Guide, Point};

pub struct Inst_ { }
pub fn prism_unit() -> Unit { mechanism::prism::<Inst_>() }
pub fn is_prism(prism: AnchoredLine) -> bool { prism[0] == prism_unit() }
pub fn find_prism(h: Handle) -> Option<AnchoredLine> { h.find_prism(prism_unit()) }
pub fn is_inst(h: Handle) -> bool { find_prism(h).is_some() }

pub fn sign(b: u8) -> bool { b == b'+' || b == b'-' }
pub fn digit(b: u8) -> bool { b.wrapping_sub(b'0') < 10 }

pub fn all_digits(s: &[u8]) -> bool {
    for i in s.iter() {
        if !digit(*i) { return false }
    }
    true
}

pub fn dig(b: u8) -> u32 { (b - b'0') as u32 }

pub fn ascii(b: u8) -> bool { (b & 0x80) == 0x00 }

pub fn read_nano(s: &[u8]) -> u32 {
    let mut x = 0u32;
    for i in s.iter() {
        let d = dig(*i);
        x = (x * 10) + d;
    }
    for _ in 0..(9 - s.len()) {
        x = x * 10;
    }
    x
}

pub fn timezone_length(s: &[u8]) -> usize {
    match s.last() {
        None => { return 0 },
        Some(b) => { if *b == b'Z' || *b == b'z' { return 1 } },
    };
    if s.len() < 6 { return 0 }
    let tz = &s[(s.len() - 6)..];
    let res = !sign(tz[0]) || !digit(tz[1]) || !digit(tz[2]) || tz[3] != b':' ||
        !digit(tz[4]) || !digit(tz[5]);
    return if res { 0 } else { 6 }
}

// Instant in time
// 1969-04-27T00:31:49.88Z (+08:40)
// 14   4  5  5  6  6  30   1 5 6
// 1980-09-21[T[hh:mm[:ss[.nnn]]][+hh:mm]]

// identify date 1980-01-01
// 1969-04-27 11:31
// 1969-04-27
pub fn parse(s: &[u8]) -> Result<Point, String> {
    if s.len() < 10 {
        return Err(format!("Bad inst, too short. An inst should start with a date, like 1985-05-24."))
    }
    if !digit(s[0]) || !digit(s[1]) || !digit(s[2]) || !digit(s[3]) || s[4] != b'-' ||
        !digit(s[5]) || !digit(s[6]) || s[7] != b'-' || !digit(s[8]) || !digit(s[9]) {
        return Err(format!("Bad inst. An inst should start with a date (YYYY-MM-DD), like 1985-05-24."))
    }
    let year =  dig(s[0]) * 1000 + dig(s[1]) * 100 + dig(s[2]) * 10 + dig(s[3]);
    let month = dig(s[5]) * 10 + dig(s[6]);
    let day =   dig(s[8]) * 10 + dig(s[9]);
    let (t, off_neg, off_hour, off_min) = {
        let rest = {
            let r = &s[10..];
            if r.is_empty() { r } else {
                let sep = r[0];
                if !(sep == b'T' || sep == b' ' || sep == b'@' || sep == b't') || r.len() == 1 {
                    return Err(format!("Bad inst, date can be followed by a separator character (T t @ space), \
                            then a time and/or timezone, like 1985-05-24T11:30:00 for example."))
                }
                &r[1..]
            }
        };
        let tz_len = timezone_length(rest);
        let t = {
            let t = &rest[..(rest.len() - tz_len)];
            if !t.is_empty() && t[t.len() - 1] == b' ' {
                &t[..(t.len() - 1)]
            } else {
                t
            }
        };
        if tz_len == 0 {
            (t, 1u32, 0, 0)
        } else if tz_len == 1 {
            (t, 0u32, 0, 0)
        } else {
            let tz = &rest[(rest.len() - 6)..];
            (t, if tz[0] == b'+' { 0u32 } else { 1u32 },
             dig(tz[1]) * 10 + dig(tz[2]), dig(tz[4]) * 10 + dig(tz[5]))
        }
    };
    // t "" "11:30" "11:30:47" "11:30:47.123"
    let (hour, min, sec, nano) = {
        let (mut hour, mut min, mut sec, mut nano) = (0, 0, 0, 0);
        let t_len = t.len();
        if 0 < t_len && t_len < 5 {
            return Err(format!("Bad inst, time should be at least HH:MM, as in 1985-05-24T11:30 for example."))
        }
        if t_len >= 5 {
            if !digit(t[0]) || !digit(t[1]) || t[2] != b':' || !digit(t[3]) || !digit(t[4]) {
                return Err(format!("Bad inst, time should start with HH:MM, \
                                    as in 1985-05-24T11:30 for example."))
            }
            hour = dig(t[0]) * 10 + dig(t[1]);
            min  = dig(t[3]) * 10 + dig(t[4]);
        }
        if t_len == 6 || t_len == 7 {
            return Err(format!("Bad inst, time should be HH:MM:SS, as in 1985-05-24T11:30:59 for example."))
        }
        if t_len >= 8 {
            if t[5] != b':' || !digit(t[6]) || !digit(t[7]) {
                return Err(format!("Bad inst, time should be HH:MM:SS, as in 1985-05-24T11:30:48 for example."))
            }
            sec = dig(t[6]) * 10 + dig(t[7]);
        }
        if t_len == 9 {
            return Err(format!("Bad inst, time should be HH:MM:SS.SS, as in 1985-05-24T11:30:55.13 for example."))
        }
        if t_len >= 10 {
            let n_digits = &t[9..];
            if t[8] != b'.' || !all_digits(n_digits) || n_digits.len() > 9 {
                return Err(format!("Bad inst, time should be HH:MM:SS.SS, as in 1985-05-24T11:30:55.13 for example."))
            }
            nano = read_nano(n_digits);
        }
        (hour, min, sec, nano)
    };
    let (month, day, hour, min, sec) = (month as u8, day as u8, hour as u8, min as u8, sec as u8);
    let (off_neg, off_hour, off_min) = (off_neg as u8, off_hour as u8, off_min as u8);
    let p = Point { year, month, day, hour, min, sec, nano, off_neg, off_hour, off_min };
    // TODO errors should show original inst text
    // let pz = subtract_offset(&p);
    // TODO fn to validate fields in range, return zeroed subtract_offset
    if is_good(&p) { Ok(p) } else { Err(format!("Bad inst, values outside acceptable range.")) }
}

pub fn is_leap(year: u32) -> bool {
    let by4 = year & 0x3 == 0;
    let cent = year % 100 == 0;
    let quad = year % 400 == 0;
    by4 && (!cent || quad)
}

pub fn last_day(month: u8, year: u32) -> u8 {
    if month == 2 {
        28 + if is_leap(year) { 1 } else { 0 }
    } else {
        let last = [31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        last[month as usize - 1]
    }
}

pub fn is_good(p: &Point) -> bool {
    if p.year >= 10_000 ||
       p.month < 1 || p.month > 12 ||
       p.day < 1 || p.day > last_day(p.month, p.year) {
        // Bad year, month, day
        return false
    }
    if p.hour > 23 || p.min > 59 || p.sec > 60 || p.nano >= 1_000_000_000 ||
       p.off_hour > 23 || p.off_min > 59 {
        // Bad hour, min, sec, nano
        return false
    }
    // TODO if leap second, validate and return zeroed point
    // Bad leap second (not at the end of the hour)
    if p.sec == 60 && p.min != 59 { return false }
    true
}
// Leap seconds so far
// 1972-06-30
// 1972-12-31
// 1973-12-31
// 1974-12-31
// 1975-12-31
// 1976-12-31
// 1977-12-31
// 1978-12-31
// 1979-12-31
// 1981-06-30
// 1982-06-30
// 1983-06-30
// 1985-06-30
// 1987-12-31
// 1989-12-31
// 1990-12-31
// 1992-06-30
// 1993-06-30
// 1994-06-30
// 1995-12-31
// 1997-06-30
// 1998-12-31
// 2005-12-31
// 2008-12-31
// 2012-06-30
// 2015-06-30
// 2016-12-31

pub fn next_day(p: &Point) -> Point {
    let mut n = *p;
    if p.day == last_day(p.month, p.year) {
        n.day = 1;
        if p.month == 12 {
            n.month = 1;
            n.year = p.year + 1;
            n
        } else {
            n.month = p.month + 1;
            n
        }
    } else {
        n.day = p.day + 1;
        n
    }
}

pub fn prev_day(p: &Point) -> Point {
    let mut n = *p;
    if p.day == 1 {
        if p.month == 1 {
            n.day = 31;
            n.month = 12;
            n.year = p.year - 1;
            n
        } else {
            let mon = p.month - 1;
            n.month = mon;
            n.day = last_day(mon, p.year);
            n
        }
    } else {
        n.day = p.day - 1;
        n
    }
}

pub fn offset_forward(p: &Point) -> Point {
    let (hour_carry, mins) = {
        let m = p.min + p.off_min;
        (m / 60, m % 60)
    };
    let (day_carry, hours) = {
        let h = p.hour + p.off_hour + hour_carry;
        (h / 24, h % 24)
    };
    let mut ret = if day_carry == 0 { *p } else { next_day(p) };
    ret.min = mins;
    ret.hour = hours;
    ret
}

pub fn offset_backward(p: &Point) -> Point {
    let (hour_carry, mins) = if p.min >= p.off_min {
        (0, p.min - p.off_min)
    } else {
        (1, 60 - (p.off_min - p.min))
    };
    let off_hours = p.off_hour + hour_carry;
    let (day_carry, hours) = if p.hour >= off_hours {
        (0, p.hour - off_hours)
    } else {
        (1, 24 - (off_hours - p.hour))
    };
    let mut ret = if day_carry == 0 { *p } else { prev_day(p) };
    ret.min = mins;
    ret.hour = hours;
    ret
}

pub fn subtract_offset(p: &Point) -> Point {
    if p.off_neg == 0 { offset_backward(p) } else { offset_forward(p) }
}

pub fn add_offset(p: &Point) -> Point {
    if p.off_neg == 0 { offset_forward(p) } else { offset_backward(p) }
}

pub fn day_of_the_week(year: u32, month: u32, day: u32) -> &'static str {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday",
                "Thursday", "Friday", "Saturday"];
    let (m, y) = if month < 3 { (month + 10, year - 1) } else { (month - 2, year) };
    let (cent, y) = (y / 100, y % 100);
    let i = day +
            (26 * m - 2) / 10 +
            y + y / 4 +
            5 * cent + cent / 4;
    days[i as usize % 7]
}

pub fn new_parsed(source: &[u8]) -> Result<Handle, String> {
    let point = match parse(source) {
        Err(msg) => { return Err(msg) },
        Ok(g) => g,
    };
    let needed = 1 /*prism*/ + Guide::units();
    let s = Segment::new(needed);
    let prism = s.line_at(0);
    prism.set(0, prism_unit());
    let guide = Guide { hash: 0, point: subtract_offset(&point), prism};
    Ok(guide.store().segment().unit().handle())
}

impl Dispatch for Inst_ { /*default tear_down, alias_components*/ }
impl Identification for Inst_ {
    fn type_name(&self) -> &'static str { "Inst" }
}

fn as_u64s(p: &Point) -> (u64, u64) {
    let d = (p.month as u64) << 40 | (p.day as u64) << 32 | p.year as u64;
    let t = (p.hour as u64) << 48 | (p.min as u64) << 40 | (p.sec as u64) << 32 | p.nano as u64;
    (d, t)
}

impl Distinguish for Inst_ {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() { return guide.hash; }

        let (d, t) = as_u64s(&guide.point);
        use hash::hash_128;
        let h = hash_128(d, t, 16);
        guide.set_hash(h).store_hash().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        let o = other.handle();
        if let Some(o_inst) = find_prism(o) {
            let p = Guide::hydrate(prism).point;
            let q = Guide::hydrate(o_inst).point;
            let ps = as_u64s(&p);
            let qs = as_u64s(&q);
            ps.0 == qs.0 && ps.1 == qs.1
        } else {
            false
        }
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        let o = other.handle();
        if let Some(o_inst) = find_prism(o) {
            let p = Guide::hydrate(prism).point;
            let q = Guide::hydrate(o_inst).point;
            let (pd, pt) = as_u64s(&p);
            let (qd, qt) = as_u64s(&q);
            return Some(pd.cmp(&qd).then(pt.cmp(&qt)))
        }
        if o.is_ref() {
            let o_prism_unit = o.logical_value()[0];
            Some(prism_unit().cmp(&o_prism_unit))
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Aggregate for Inst_ { }
impl Sequential for Inst_ { }
impl Associative for Inst_ { }
impl Reversible for Inst_ {}
impl Sorted for Inst_ {}

pub fn width_digits(nano: u32) -> (usize, u32) {
    let mut x = nano;
    for i in 0..9 {
        if x % 10 != 0 { return (9 - i, x) }
        x = x / 10;
    }
    (0, 0)
}

impl Notation for Inst_ {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        // 1969-04-27T00:31:49+08:40
        let guide = Guide::hydrate(prism);
        let p = add_offset(&guide.point);
        write!(f, "#inst \"{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
               p.year, p.month, p.day, p.hour, p.min, p.sec)?;
        if p.nano != 0 {
            let (width, digits) = width_digits(p.nano);
            write!(f, ".{:0w$}", digits, w = width)?;
        }
        write!(f, "{}{:02}:{:02}\"",
               if p.off_neg == 0 { '+' } else { '-' }, p.off_hour, p.off_min)
    }
}
impl Numeral for Inst_ {}
impl Callable for Inst_ {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_of_the_week_ex() {
        assert_eq!("Wednesday", day_of_the_week(2021, 9, 15));
        assert_eq!("Thursday", day_of_the_week(1863, 1, 1)); // Emancipation Proclamation
        assert_eq!("Friday", day_of_the_week(1620, 12, 18)); // Plymouth Rock
        assert_eq!("Wednesday", day_of_the_week(1869, 11, 17)); // Suez Canal
        assert_eq!("Tuesday", day_of_the_week(1966, 9, 6)); // Star Trek
        assert_eq!("Friday", day_of_the_week(1844, 5, 24)); // Morse Telegraph
        assert_eq!("Sunday", day_of_the_week(1989, 3, 12)); // WWW
    }
}
