//! A cohesive fressian library for rust
use std::fmt;

mod memory;
mod map;
mod vector;
mod bit;
mod value;

use value::Value;

/**
Methods on Value are the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- &Value into another Value (during its scope)
- Special high level operations like split

Types:
- Atomics
 - boolean
 - nil
 - char
 - string
 - symbol
 - keyword
 - integral
 - rational
 - float point
- Collections
 - List
 - Vector
 - Map
 - Set
 - SortedMap
 - SortedSet

Common Rust traits:
Clone
PartialEq
PartialOrd
Hash
Default
Numeric traits galore
Index
Fn
Display
Drop
IntoIterator
From/Into
Send/Sync
*/


// Data and vtable for trait
struct SeqValue {}
struct CollValue {}
struct AssociativeValue {}
struct SequentialValue {}
struct SortedValue {}
struct NumericValue {}

// Constructed via typecheck on Value
// Static dispatch to methods
struct MapValue {}
struct SortedMapValue {}
struct SetValue {}
struct SortedSetValue {}
struct VectorValue {}
struct ListValue {}
struct StringValue {}
struct Symbol {}
struct Keyword {}
struct Integral {}
struct Rational {}
struct FloatPoint {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_immediate() {
        assert!(Value {handle: 7}.is_immediate())
    }

    #[test]
    fn is_not() {
        assert!(Value::NIL.is_not() && Value::FALSE.is_not())
    }

    #[test]
    fn is_so() {
        assert!(Value {handle: 0}.is_so())
    }

    #[test]
    fn is_nil() {
        assert!(Value {handle: 7}.is_nil())
    }

    #[test]
    fn is_true() {
        assert!(Value {handle: !0}.is_true())
    }

    #[test]
    fn is_false() {
        assert!(Value {handle: !0 - 8}.is_false())
    }

    #[test]
    fn is_immediate_number() {
        assert!(Value {handle: 1}.is_immediate_number() &&
        Value {handle: 5}.is_immediate_number())
    }
}
