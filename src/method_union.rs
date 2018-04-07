//! Define named unions of methods.
//!


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
