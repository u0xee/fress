// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.
use std::cell::Cell;


// From George Marsaglia's "Xorshift RNGs"
pub fn cycle(mut y: u64) -> u64 {
    y ^= y << 13;
    y ^= y >> 7;
    y ^= y << 17;
    y
}

thread_local! {
    pub static PRN: Cell<u64> = Cell::new(0);
}

pub fn next_random() -> u64 {
    PRN.with(|y| {
        let ret = y.get();
        y.set(cycle(ret));
        ret
    })
}


pub mod dummy {
    use std::fmt;
    use memory::*;
    use dispatch::*;
    use Value;

    pub struct DummyValue {
        prism: Unit,
    }
    impl DummyValue {
        pub fn new() -> Unit {
            let mut s = Segment::new(1);
            s[1] = prism::<DummyValue>();
            Unit::from(s)
        }
    }
    impl Dispatch for DummyValue {
        fn tear_down(&self) {
            let p: Line = Unit::from(&self.prism as *const Unit).into();
            let mut s: Segment = p.offset(-1).into();
            if s.unalias() == 0 {
                Segment::free(s);
            }
        }
    }
    impl fmt::Display for DummyValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }
    impl Identification for DummyValue {}
    impl Distinguish for DummyValue {}
    impl Aggregate for DummyValue {}
    impl Sequential for DummyValue {}
    impl Associative for DummyValue {}
    impl Reversible for DummyValue {}
    impl Sorted for DummyValue {}
    impl Named for DummyValue {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(next_random(), 0);
        assert_eq!(next_random(), 0);
    }
    #[test]
    fn try_dummy() {
        let d = dummy::DummyValue::new();
        use memory::*;
        use dispatch::*;
        Segment::from(d).tear_down()
    }
}
