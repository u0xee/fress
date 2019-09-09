// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use std::str::FromStr;
use edn::reader::{EdnReader, ReadResult};
use edn;

impl From<bool> for Value {
    fn from(x: bool) -> Value {
        if x { Handle::tru().value() } else { Handle::fals().value() }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        (&self).into()
    }
}

impl Into<bool> for &Value {
    fn into(self) -> bool {
        self.handle().is_so()
    }
}

impl From<char> for Value {
    fn from(x: char) -> Self {
        use character::Character;
        Character::new(x).value()
    }
}

impl Into<char> for Value {
    fn into(self) -> char {
        use character::{Character, as_char};
        as_char(Character::prism(self.handle()).unwrap())
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        use integral::Integral;
        Integral::new_value(x)
    }
}

impl From<i32> for Value {
    fn from(x: i32) -> Self { Value::from(x as i64) }
}
impl From<i16> for Value {
    fn from(x: i16) -> Self { Value::from(x as i64) }
}
impl From<i8> for Value {
    fn from(x: i8) -> Self { Value::from(x as i64) }
}
impl From<isize> for Value {
    fn from(x: isize) -> Self { Value::from(x as i64) }
}
impl From<u64> for Value {
    fn from(x: u64) -> Self { Value::from(x as i64) }
}
impl From<u32> for Value {
    fn from(x: u32) -> Self { Value::from(x as i64) }
}
impl From<u16> for Value {
    fn from(x: u16) -> Self { Value::from(x as i64) }
}
impl From<u8> for Value {
    fn from(x: u8) -> Self { Value::from(x as i64) }
}
impl From<usize> for Value {
    fn from(x: usize) -> Self { Value::from(x as i64) }
}

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        use float_point::FloatPoint;
        FloatPoint::new(x).handle().value()
    }
}
impl From<f32> for Value {
    fn from(x: f32) -> Self { Value::from(x as f64) }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        use string::Str;
        Str::new_value_from_str(s)
    }
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let mut reader = EdnReader::new();
        match edn::read(&mut reader, b) {
            ReadResult::Ok { bytes_used, value } => {
                Ok(value.handle().value())
            },
            ReadResult::NeedMore { bytes_not_used } => {
                let trailing_space = {
                    let mut v = Vec::new();
                    let i = b.len() - bytes_not_used as usize;
                    v.extend_from_slice(&b[i..]);
                    v.push(0x20u8);
                    v
                };
                match edn::read(&mut reader, &trailing_space[..]) {
                    ReadResult::Ok { bytes_used, value } => {
                        Ok(value.handle().value())
                    },
                    ReadResult::NeedMore { bytes_not_used } => {
                        Err(format!("Incomplete edn element: {:?}", s))
                    },
                    ReadResult::Error { location, message } => {
                        Err(message)
                    },
                }
            },
            ReadResult::Error { location, message } => {
                Err(message)
            },
        }
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(option: Option<T>) -> Value {
        match option {
            Some(t) => t.into(),
            None    => Handle::nil().value(),
        }
    }
}

// vector, array, slice
impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(val: Vec<T>) -> Value {
        unimplemented!()
    }
}

// tuples
impl<A, B> From<(A, B)> for Value {
    fn from(x: (A, B)) -> Self {
        unimplemented!()
    }
}
