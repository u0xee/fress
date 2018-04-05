use std::fmt;
use memory;
use bit::{bottom_32, top_32};
use Value;
use Aggregate;

pub const FIELD_COUNT: isize = 1;
pub const FIELD_META: isize = 2;
pub const FIELD_HASH: isize = 3;
pub const FIELD_POP: isize = 4;
pub const FIELD_TAIL: isize = 5;

struct MapFields {
    base: *mut u64,
}

impl MapFields {
    fn count(&self) -> u64 {
        unsafe {
            bottom_32(*self.base.offset(FIELD_COUNT))
        }
    }

    fn meta(&self) -> u64 {
        unsafe {
            *self.base.offset(FIELD_META)
        }
    }

    fn hash(&self) -> u64 {
        unsafe {
            bottom_32(*self.base.offset(FIELD_HASH))
        }
    }

    fn population(&self) -> u64 {
        unsafe {
            bottom_32(*self.base.offset(FIELD_POP))
        }
    }

    fn population_leaf(&self) -> u64 {
        unsafe {
            top_32(*self.base.offset(FIELD_POP))
        }
    }

    fn tail(&self) -> &[u64] {
        unsafe {
            let length = self.population().count_ones() * 2;
            use std::slice::from_raw_parts;
            from_raw_parts(self.base.offset(FIELD_TAIL), length as usize)
        }
    }
}

pub static MAP_SENTINEL: u8 = 0;
pub struct Map {
    base: u64,
}

impl Aggregate for Map {
    fn conj(&mut self, v: Value) -> Value {
        let mf = MapFields {base: &mut self.base as *mut u64};
        let base_ptr = &mut self.base;
        let erased_ptr = base_ptr as *mut u64;
        unsafe {
            *erased_ptr = 14;
            *erased_ptr.offset(1) = 15
        }
        Value::NIL
    }
}


// Basic operations: struct as u64, impl treating struct as base address
// Using ptr as struct reference, then trait object. Calling object methods.
// Extracting vtable pointer from trait object.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_erase() {
        let mut v: Vec<u64> = Vec::with_capacity(2);
        unsafe {
            v.set_len(2);
        }
        let mut p = v.as_mut_ptr();
        let mut m = p as *mut Map;
        let result = unsafe {
            let mut mref = &mut *m;
            let mref_as_u64 = mref as *const Map as u64;
            let mut magg = mref as &mut Aggregate;
            use std::mem;

            assert_eq!(mem::size_of::<&Map>(), 8);
            //assert_eq!(;::size_of::<&Aggregate>(), 16);
            let res = magg.conj(Value::FALSE);
            let raw_ptrs = unsafe {mem::transmute::<&Aggregate, [u64;2]>(magg)};
            assert_eq!(raw_ptrs[0], mref_as_u64);
            res
        };
        assert!(result.is_nil());
        assert_eq!(v[0], 14);
        assert_eq!(v[1], 15);
    }
}
