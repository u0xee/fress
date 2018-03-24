use std::fmt;
use memory;
use bit::{bottom_32, top_32, MASK_32};
use Dispatch;
use Value;

pub const FIELD_COUNT: isize = 1;
pub const FIELD_META: isize = 2;
pub const FIELD_HASH: isize = 3;
pub const FIELD_ROOT: isize = 4;
pub const FIELD_TAIL: isize = 5;

struct VectorFields {
    base: *mut u64,
}

impl VectorFields {
    fn count(&self) -> u64 {
        unsafe {
            bottom_32(*self.base.offset(FIELD_COUNT))
        }
    }

    fn set_count(&mut self, c: u64) {
        unsafe {
            let field = *self.base.offset(FIELD_COUNT);
            let with_c = (field & !MASK_32) | bottom_32(c);
            *self.base.offset(FIELD_COUNT) = with_c;
        }
    }

    fn meta(&self) -> u64 {
        unsafe {
            *self.base.offset(FIELD_META)
        }
    }

    fn set_meta(&mut self, m: u64) {
        unsafe {
            *self.base.offset(FIELD_META) = m;
        }
    }

    fn hash(&self) -> u64 {
        unsafe {
            bottom_32(*self.base.offset(FIELD_HASH))
        }
    }

    fn set_hash(&mut self, h: u64) {
        unsafe {
            let field = *self.base.offset(FIELD_HASH);
            let with_h = (field & !MASK_32) | bottom_32(h);
            *self.base.offset(FIELD_HASH) = with_h;
        }
    }

    fn root(&self) -> u64 {
        unsafe {
            *self.base.offset(FIELD_ROOT)
        }
    }

    fn set_root(&mut self, r: u64) {
        unsafe {
            *self.base.offset(FIELD_ROOT) = r;
        }
    }

    fn tail(&self) -> u64 {
        unsafe {
            1
        }
    }

    fn set_tail(&mut self, t: u64) {
        unsafe {
            *self.base.offset(FIELD_TAIL) = t;
        }
    }
}

pub static VECTOR_SENTINEL: u8 = 0;
pub struct Vector {
    base: u64,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Dispatch for Vector {
    fn type_name(&self) -> String {
        "Vector".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }

    fn hash(&self) -> u32 {
        unimplemented!()
    }

    fn eq(&self, other: &Dispatch) -> bool {
        unimplemented!()
    }
}

impl Vector {
    fn store_trait_table(location: *mut u64) {
        unsafe {
            let mut vector_ref = &mut *(location as *mut Vector);
            let mut dispatch_object = vector_ref as &mut Dispatch;
            use std::mem::transmute;
            let location_table = transmute::<&mut Dispatch, [u64;2]>(dispatch_object);
            assert_eq!(location as u64, location_table[0]);
            *location = location_table[1];
        }
    }

    pub fn new() -> Value {
        let cap = 16;
        let base_ptr = memory::space_for(cap);
        let dispatch_ptr = unsafe { base_ptr.offset(1) };
        Vector::store_trait_table(dispatch_ptr);
        let mut fields = VectorFields {base: dispatch_ptr };
        fields.set_count(0);
        fields.set_meta(0);
        fields.set_hash(0);
        fields.set_root(0);
        fields.set_tail(0);
        Value { handle: base_ptr as usize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let imm = Value::NIL;
        assert_eq!("Immediate Value".to_string(), imm.type_name());
        let v = Vector::new();
        assert_eq!("Vector".to_string(), v.type_name());
    }
}
