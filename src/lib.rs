//! A cohesive fressian library for rust
use std::fmt;

mod memory;
mod map;
mod vector;
mod bit;

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub struct Value {
    handle: usize,
}

impl Value {
    fn split(self) -> (Value, Value) {
        // TODO support non immediate values
        (Value {handle: self.handle}, Value {handle: self.handle})
    }

    fn is_immediate(&self) -> bool {
        self.handle & 0x01 == 0x01
    }

    fn is_not(&self) -> bool {
        self.handle & 0x0F == 0x07
    }

    fn is_so(&self) -> bool {
        !self.is_not()
    }

    // TODO associated constants may not be good. factory fn?
    const NIL: Value = Value {handle: 0x07};

    fn is_nil(&self) -> bool {
        self.handle == 0x07
    }

    const TRUE: Value = Value {handle: std::usize::MAX};

    fn is_true(&self) -> bool {
        self.handle == Value::TRUE.handle
    }

    const FALSE: Value = Value {handle: !0x08};

    fn is_false(&self) -> bool {
        self.handle == std::usize::MAX & !8
    }

    fn is_char(&self) -> bool {
        self.handle & 0x07 == 0x03
    }

    fn is_string(&self) -> bool {
        // TODO separate char immediate and string
        self.handle & 0x07 == 0x03
    }

    fn is_immediate_number(&self) -> bool {
        self.handle & 0x03 == 0x01
    }

    fn is_number(&self) -> bool {
        // TODO support boxed numbers
        self.is_immediate_number()
    }

    fn as_pointer(&self) -> *mut () {
        self.handle as *mut ()
    }

    fn as_i64(&self) -> i64 {
        (self.handle as i64) >> 3
    }
}

impl Value {
    pub fn type_name(&self) -> String {
        if self.is_immediate() {
            "Immediate Value".to_string()
        } else {
            unsafe {
                let table_ptr = (self.handle as *const u64).offset(1);
                let t_object: [u64; 2] = [table_ptr as u64, *table_ptr];
                use std::mem::transmute;
                let dispatch = transmute::<[u64;2], &Dispatch>(t_object);
                let s = dispatch.type_name();
                use std::mem::forget;
                //forget(dispatch);
                s
            }
        }
    }
    fn map_value(&self) {
        panic!("{} is NOT a MapValue", self.type_name())
    }
    fn set_value(&self) {
        panic!("{} is NOT a SetValue", self.type_name())
    }
    fn vector_value(&self) {
        panic!("{} is NOT a VectorValue", self.type_name())
    }
    fn list_value(&self) {
        panic!("{} is NOT a ListValue", self.type_name())
    }
    fn string_value(&self) {
        panic!("{} is NOT a StringValue", self.type_name())
    }
    fn symbol(&self) {
        panic!("{} is NOT a Symbol", self.type_name())
    }
    fn keyword(&self) {
        panic!("{} is NOT a Keyword", self.type_name())
    }
    fn integral(&self) {
        panic!("{} is NOT an Integral", self.type_name())
    }
    fn rational(&self) {
        panic!("{} is NOT a Rational", self.type_name())
    }
    fn float_point(&self) {
        panic!("{} is NOT a FloatPoint", self.type_name())
    }
}

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

/// A trait to dynamically dispatch methods on heap values
trait Dispatch : fmt::Display {
    fn type_name(&self) -> String;
    fn type_sentinel(&self) -> *const u8;
    fn hash(&self) -> u32;
    fn eq(&self, other: &Dispatch) -> bool;

    fn conj(&mut self, x: Value) -> Value {
        panic!("Can't conj onto a {}", self.type_name())
    }
    fn empty(&mut self) -> Value {
        panic!("Can't call empty on a {}", self.type_name())
    }
    fn first(&self) {
        panic!("Can't call first on a {}", self.type_name())
    }
    fn rest(&self) {
        panic!("Can't call rest on a {}", self.type_name())
    }
    fn count(&self) {
        panic!("Can't count a {}", self.type_name())
    }
    fn get(&self) {
        panic!("Can't call get on a {}", self.type_name())
    }
    fn nth(&self) {
        panic!("Can't call nth on a {}", self.type_name())
    }

    fn seq_value(&self) -> &Seq {
        panic!("{} is NOT a SeqValue", self.type_name())
    }
    fn coll_value(&self) -> &Coll {
        panic!("{} is NOT a CollValue", self.type_name())
    }
    fn associative_value(&self) -> &Associative {
        panic!("{} is NOT an AssociativeValue", self.type_name())
    }
    fn sequential_value(&self) -> &Sequential {
        panic!("{} is NOT a SequentialValue", self.type_name())
    }
    fn sorted_value(&self) -> &Sorted {
        panic!("{} is NOT a SortedValue", self.type_name())
    }
    fn numeric_value(&self) -> bool {
        panic!("{} is NOT a NumericValue", self.type_name())
    }
}

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


trait Aggregate {
    fn conj(&mut self, v: Value) -> Value;
}
trait Seqable {
    fn seq(&self) -> Value;
}
trait Seq : Aggregate + Seqable {
    fn first(&self) -> Value;
    fn rest(&self) -> Value;
}
trait Coll : Seq {
    fn count(&self) -> u32;
    fn empty(&self) -> Value;
    fn meta(&self) -> Value;
    fn with_meta(&self) -> Value;
}
trait Associative : Coll {
    fn get(&self, k: Value) -> Value;
    fn contains(&self, k: Value) -> bool;
    fn assoc(&self, k: Value, v: Value) -> Value;
    fn dissoc(&self, k: Value) -> Value;
}
trait Sequential : Coll {
    fn nth(&self, idx: i64) -> Value;
    fn peek(&self) -> Value;
    fn pop(&self) -> Value;
}
trait Reversible {
    fn rseq(&self) -> Value;
}
trait Sorted : Associative + Reversible {
    fn subseq(&self, start: Value, end: Value) -> Value;
    fn rsubseq(&self, start: Value, end: Value) -> Value;
}
trait Named {
    fn name(&self) -> &str;
    fn namespace(&self) -> &str;
}
trait Deref {
    fn deref(&self) -> Value;
    fn deref_timeout(&self, time: u64) -> Value;
}
trait Atom : Deref {
    fn swap(&self, f: &Fn(Value) -> Value);
}


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
