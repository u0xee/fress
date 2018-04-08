/// A trait to dynamically dispatch methods on heap values

trait Dispatch :
Identification +
Contrast +
AggregateAbstractions +
StreamlinedMethods {}

trait Identification : fmt::Display {
    fn type_name(&self) -> String;

    fn type_sentinel(&self) -> *const u8;
}

trait Contrast {
    fn hash(&self) -> u32;

    fn eq(&self, other: &Dispatch) -> bool;
}

trait AggregateAbstractions : Identification {
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

trait StreamlinedMethods : Identification {
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
}
