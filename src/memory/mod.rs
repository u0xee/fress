// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::mem;

pub mod unit;
mod anchor;
pub mod segment;
mod thread;
mod degree;

/*
Memory System API
- Request memory
- Register interest in memory (during setup of new memory or update of old mem)
- Deregister interest in memory (during Drop of memory or update of old mem)
- During "update", determine when memory is not shared
 - Determine capacity, then:
  - Can be destructively updated
  - Can have its parts "moved" into larger (or smaller) capacity memory

Local Memory Interest Table
Thread local associative structure. Capacity, no dispatch, meta, or hash.
K - Pointer to memory
V - Shared? and Local Count
*/

use self::unit::Unit;

pub fn set(address: *const Unit, u: Unit) {
    unsafe {
        *(address as *mut Unit) = u;
    }
}

pub fn get(address: *const Unit) -> Unit {
    unsafe {
        *address
    }
}


pub fn interest_in(values: &[Value]) {
    for v in values {
        if Unit::from(v).is_even() {
            register_interest(Unit::from(v).into())
        }
    }
}

pub fn register_interest(ptr: *const u64) {
    unimplemented!()
}

pub fn deregister_interest(ptr: *const u64) {
    unimplemented!()
}

pub fn is_shared(ptr: *const u64) -> bool {
    false
}



#[cfg(test)]
mod test {
    use super::*;
    use super::unit::Unit;

    #[test]
    fn unit_from_f32() {
        let f: f32 = 5.4;
        let u: Unit = f.into();
        let ff: f32 = u.into();
        assert_eq!(f, ff)
    }
}
