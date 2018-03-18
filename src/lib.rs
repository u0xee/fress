//! Use fressian data in Rust

mod memory;

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
struct Value {
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

/**
Trie representation. Similarities - associative, local memory interest table,
vector nodes.

Capacity Dispatch Count Meta Hash Pop Tail
Capacity Tail

Capacity Dispatch Count Meta Hash Root Tail
Capacity Tail

Layout of fields by constant indices. Getters, query helpers, inserts removes.

Methods on Value are the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- ValueView into another Value (during its scope)
- AssociativeValue, etc trait object specializations
- Special high level operations like split

Types:
- Atomics
 - boolean
 - nil
 - char
 - string
 - symbol
 - keyword
 - regex
 - integral
 - float
 - ratio
- Collections
 - List
 - Vector
 - Map
 - Set
 - SortedMap
 - SortedSet
- Other
 - Seq
 - Atom

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
trait Dispatch {
    fn hash();
    fn eq(); // lt, gt
    fn fmt(); // to_string()
    fn cast_to_type();
    fn cast_to_trait_object();
}

trait Aggregate {
    fn conj(&self, v: Value);
}
trait Seqable {
    fn seq(&self) -> &'static Seq;
}
trait Seq : Aggregate + Seqable {
    fn first(&self);
    fn rest(&self);
}
trait Coll : Seq {
    fn count();
    fn meta(&self);
    fn with_meta(&self);
}
trait Associative : Coll {
    fn get();
    fn contains();
    fn assoc();
    fn dissoc();
}
trait Sequential : Coll {
    fn nth();
    fn peek();
    fn pop();
}
trait Reversible {
    fn rseq();
}
trait Sorted : Associative + Reversible {
    fn subseq();
    fn rsubseq();
}
trait Named {
    // Keyword and Symbol only
    fn name();
    fn namespace();
}
// Numeric traits?
trait Deref {
    fn deref();
    fn deref_timeout();
}
trait Atom : Deref {
    fn swap();
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
