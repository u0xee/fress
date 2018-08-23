//! Value handle - bit patterns:
//!
//! !(handle | 0x08) => 0, boolean
//! 0xFFFFFFFFFFFFFFFF true
//! 0xFFFFFFFFFFFFFFF7 false
//!
//! End in 0b0111, logically negative
//! 0x0000000000000007 nil
//! 0xFFFFFFFFFFFFFFF7 false
//!
//! End in 0b011
//! 0xXXXXXXXX00000003 char
//! 0xXXXXXXXXXXXXXXLB string, L holds count
//!
//! End in 0b001
//! 0xXXXXXXXXXXXXXXX1 integral
//! 0xXXXXXXXXXXXXXXX9 FloatPoint
//!
//! Even handles (rightmost bit of 0) are pointers.
//! They point to segments that have a distributor as the first unit.
//!

pub mod associative;
pub mod boolean;
pub mod character;
pub mod coll;
pub mod float_point;
pub mod integral;
pub mod keyword;
pub mod list;
pub mod map;
pub mod numeric;
pub mod rational;
pub mod seq;
pub mod sequential;
pub mod set;
pub mod sorted_map;
pub mod sorted;
pub mod sorted_set;
pub mod string;
pub mod symbol;
pub mod vector;

pub use self::vector::VectorValue;
use Value;


impl Value {
    pub fn is_immediate(&self) -> bool {
        !self.handle.is_even()
    }

    pub fn is_not(&self) -> bool {
        self.handle.u() & 0x0F == 0x07
    }

    pub fn is_so(&self) -> bool {
        !self.is_not()
    }

    pub fn is_nil(&self) -> bool {
        self.handle == Value::NIL.handle
    }

    pub fn is_true(&self) -> bool {
        self.handle == Value::TRUE.handle
    }

    pub fn is_false(&self) -> bool {
        self.handle == Value::FALSE.handle
    }

    pub fn is_char(&self) -> bool {
        self.handle.u() & 0x07 == 0x03
    }

    pub fn is_string(&self) -> bool {
        // TODO separate char immediate and string
        self.handle.u() & 0x07 == 0x03
    }

    pub fn is_immediate_number(&self) -> bool {
        self.handle.u() & 0x07 == 0x01
    }

    pub fn is_integral(&self) -> bool {
        self.handle.u() & 0x0F == 0x01
    }

    pub fn is_float_point(&self) -> bool {
        self.handle.u() & 0x0F == 0x09
    }

    pub fn is_number(&self) -> bool {
        // TODO support boxed numbers
        self.is_immediate_number()
    }
}
// casts to specific types

// Constructed via typecheck on Value
// Static dispatch to methods
struct MapValue {}
struct SortedMapValue {}
struct SetValue {}
struct SortedSetValue {}

struct ListValue {}
struct StringValue {}
struct Boolean {}
struct Symbol {}
struct Keyword {}
struct Integral {}
struct Rational {}
struct FloatPoint {}

// Data and vtable for trait
struct SeqValue {}
struct CollValue {}
struct AssociativeValue {}
struct SequentialValue {}
struct SortedValue {}
struct NumericValue {}
