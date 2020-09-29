// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

// Add
impl ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value { self.consume().add(rhs.consume()).value() }
}
impl<'a> ops::Add<&'a Value> for Value {
    type Output = Value;
    fn add(self, rhs: &Value) -> Value { self + rhs.split_out() }
}
impl<'a> ops::Add<Value> for &'a Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value { self.split_out() + rhs }
}
impl<'a> ops::Add for &'a Value {
    type Output = Value;
    fn add(self, rhs: &Value) -> Value { self.split_out() + rhs.split_out() }
}
// Subtract
impl ops::Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value { self.consume().sub(rhs.consume()).value() }
}
impl<'a> ops::Sub<&'a Value> for Value {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Value { self - rhs.split_out() }
}
impl<'a> ops::Sub<Value> for &'a Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value { self.split_out() - rhs }
}
impl<'a> ops::Sub for &'a Value {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Value { self.split_out() - rhs.split_out() }
}
// Multiply
impl ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value { self.consume().mul(rhs.consume()).value() }
}
impl<'a> ops::Mul<&'a Value> for Value {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Value { self * rhs.split_out() }
}
impl<'a> ops::Mul<Value> for &'a Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value { self.split_out() * rhs }
}
impl<'a> ops::Mul for &'a Value {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Value { self.split_out() * rhs.split_out() }
}
// Divide
impl ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value { self.consume().div(rhs.consume()).value() }
}
impl<'a> ops::Div<&'a Value> for Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Value { self / rhs.split_out() }
}
impl<'a> ops::Div<Value> for &'a Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value { self.split_out() / rhs }
}
impl<'a> ops::Div for &'a Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Value { self.split_out() / rhs.split_out() }
}
// Remainder
impl ops::Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Value) -> Value { self.consume().rem(rhs.consume()).value() }
}
impl<'a> ops::Rem<&'a Value> for Value {
    type Output = Value;
    fn rem(self, rhs: &Value) -> Value { self % rhs.split_out() }
}
impl<'a> ops::Rem<Value> for &'a Value {
    type Output = Value;
    fn rem(self, rhs: Value) -> Value { self.split_out() % rhs }
}
impl<'a> ops::Rem for &'a Value {
    type Output = Value;
    fn rem(self, rhs: &Value) -> Value { self.split_out() % rhs.split_out() }
}
// Negate
impl ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Value { self.consume().neg().value() }
}
impl<'a> ops::Neg for &'a Value {
    type Output = Value;
    fn neg(self) -> Value { -(self.split_out()) }
}
// Not
impl ops::Not for Value {
    type Output = Value;
    fn not(self) -> Value { !(&self) }
}
impl<'a> ops::Not for &'a Value {
    type Output = Value;
    fn not(self) -> Value {
        if self.handle().is_not() {
            Handle::tru().value()
        } else {
            Handle::fals().value()
        }
    }
}
// BitAnd
impl ops::BitAnd for Value {
    type Output = Value;
    fn bitand(self, rhs: Value) -> Value { self.consume().bitand(rhs.consume()).value() }
}
impl<'a> ops::BitAnd<&'a Value> for Value {
    type Output = Value;
    fn bitand(self, rhs: &Value) -> Value { self & rhs.split_out() }
}
impl<'a> ops::BitAnd<Value> for &'a Value {
    type Output = Value;
    fn bitand(self, rhs: Value) -> Value { self.split_out() & rhs }
}
impl<'a> ops::BitAnd for &'a Value {
    type Output = Value;
    fn bitand(self, rhs: &Value) -> Value { self.split_out() & rhs.split_out() }
}
// BitOr
impl ops::BitOr for Value {
    type Output = Value;
    fn bitor(self, rhs: Value) -> Value { self.consume().bitor(rhs.consume()).value() }
}
impl<'a> ops::BitOr<&'a Value> for Value {
    type Output = Value;
    fn bitor(self, rhs: &Value) -> Value { self | rhs.split_out() }
}
impl<'a> ops::BitOr<Value> for &'a Value {
    type Output = Value;
    fn bitor(self, rhs: Value) -> Value { self.split_out() | rhs }
}
impl<'a> ops::BitOr for &'a Value {
    type Output = Value;
    fn bitor(self, rhs: &Value) -> Value { self.split_out() | rhs.split_out() }
}
// BitXor
impl ops::BitXor for Value {
    type Output = Value;
    fn bitxor(self, rhs: Value) -> Value { self.consume().bitxor(rhs.consume()).value() }
}
impl<'a> ops::BitXor<&'a Value> for Value {
    type Output = Value;
    fn bitxor(self, rhs: &Value) -> Value { self ^ rhs.split_out() }
}
impl<'a> ops::BitXor<Value> for &'a Value {
    type Output = Value;
    fn bitxor(self, rhs: Value) -> Value { self.split_out() ^ rhs }
}
impl<'a> ops::BitXor for &'a Value {
    type Output = Value;
    fn bitxor(self, rhs: &Value) -> Value { self.split_out() ^ rhs.split_out() }
}
// Shift Left
impl ops::Shl<u32> for Value {
    type Output = Value;
    fn shl(self, rhs: u32) -> Value { self.consume().shl(rhs).value() }
}
impl<'a> ops::Shl<u32> for &'a Value {
    type Output = Value;
    fn shl(self, rhs: u32) -> Value { self.split_out() << rhs }
}
// Shift Right
impl ops::Shr<u32> for Value {
    type Output = Value;
    fn shr(self, rhs: u32) -> Value { self.consume().shr(rhs).value() }
}
impl<'a> ops::Shr<u32> for &'a Value {
    type Output = Value;
    fn shr(self, rhs: u32) -> Value { self.split_out() << rhs }
}

