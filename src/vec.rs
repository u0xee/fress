use std::fmt;
use memory;
use bit::{bottom_32, top_32, top_16, MASK_32, MASK_16, top_byte, with_top_byte, second_top_byte, clear_top, clear_bottom, splice, with_second_top_byte};
use Dispatch;
use Value;

pub const FIELD_COUNT: isize = 1;
pub const FIELD_META: isize = 2;
pub const FIELD_HASH: isize = 3;
pub const FIELD_ROOT: isize = 4;
pub const FIELD_TAIL: isize = 5;

pub const BITS: u32 = 5; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

struct VectorFields {
    dispatch: *mut u64,
}

impl VectorFields {
    fn dispatch_offset(&self) -> u8 {
        unsafe {
            top_byte(*self.dispatch.offset(FIELD_COUNT))
        }
    }

    fn set_dispatch_offset(&mut self, o: u8) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) = with_top_byte(field, o);
        }
    }

    fn capacity_ptr(&self) -> *const u64 {
        unsafe {
            (self.dispatch as *const u64).offset(-(self.dispatch_offset() as isize))
        }
    }

    fn count(&self) -> u32 {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            clear_top(field, self.non_count_bits())
        }
    }

    fn set_count(&mut self, c: u32) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) =
                splice(field, c, self.non_count_bits());
        }
    }

    fn non_count_bits(&self) -> u8 {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            second_top_byte(field)
        }
    }

    fn set_non_count_bits(&mut self, bits: u8) {
        // I expect bits to take on values 32 and 56, for 4 and 1 byte counts
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) = with_second_top_byte(field, bits);
        }
    }

    fn inline_tail_hash(&self) -> u32 {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            field >> 16 // as u32
        }
    }

    fn set_inline_tail_hash(&mut self, h: u32) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) =
                clear_bottom(field, 48) |
                    clear_top(field, 48) |
                    ((h as u64) << 16);
        }
    }

    fn inline_tail_has_meta(&self) -> u8 {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            field << 48 >> 56 // as u8
        }
    }

    fn set_inline_tail_has_meta(&mut self, b: u8) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) =
                clear_bottom(field, 16) |
                    clear_top(field, 56) |
                    ((b as u64) << 8);
        }
    }

    fn inline_tail(&self) -> &[u64] {
        unsafe {
            use std::slice;
            let first = self.dispatch.offset(FIELD_COUNT + self.inline_tail_has_meta() + 1);
            slice::from_raw_parts(first, self.count())
        }
    }

    fn meta(&self) -> u64 {
        unsafe {
            *self.dispatch.offset(FIELD_META)
        }
    }

    fn set_meta(&mut self, m: u64) {
        unsafe {
            *self.dispatch.offset(FIELD_META) = m;
        }
    }

    fn hash(&self) -> u64 {
        unsafe {
            bottom_32(*self.dispatch.offset(FIELD_HASH))
        }
    }

    fn set_hash(&mut self, h: u64) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_HASH);
            let with_h = (field & !MASK_32) | bottom_32(h);
            *self.dispatch.offset(FIELD_HASH) = with_h;
        }
    }

    fn root(&self) -> u64 {
        unsafe {
            *self.dispatch.offset(FIELD_ROOT)
        }
    }

    fn set_root(&mut self, r: u64) {
        unsafe {
            *self.dispatch.offset(FIELD_ROOT) = r;
        }
    }

    fn tail(&self) -> u64 {
        unsafe {
            *self.dispatch.offset(FIELD_TAIL)
        }
    }

    fn set_tail(&mut self, t: u64) {
        unsafe {
            *self.dispatch.offset(FIELD_TAIL) = t;
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

impl Drop for Vector {
    fn drop(&mut self) {
        unimplemented!()
    }
}

fn tree_count(count: u32) -> u32 {
    (count - 1) & !MASK
}

fn significant_bits(x: u32) -> u8 {
    /*bits in a u32*/ 32 - x.leading_zeros()
}

fn digit_count(x: u32) -> u8 {
    (significant_bits(x) + BITS - 1) / BITS
}

fn digit(x: u32, idx: u8) -> u8 {
    (x >> (idx * BITS)) as u8
}

fn digit_iter(x: u32, digits: u8) {
    // Digit iterator struct
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
        let base_ptr = memory::space(memory::CACHE_LINE);
        let dispatch_ptr = unsafe { base_ptr.offset(1) };
        Vector::store_trait_table(dispatch_ptr);
        let mut fields = VectorFields { dispatch: dispatch_ptr };
        fields.set_dispatch_offset(1);
        fields.set_non_count_bits(56);
        fields.set_count(0);
        fields.set_inline_tail_hash(0);
        fields.set_inline_tail_has_meta(0);
        Value { handle: base_ptr as usize }
    }

    fn conj(&mut self, x: Value) -> Value {
        let mut fields = VectorFields { dispatch: &mut self.base as *mut u64 };
        let count = fields.count();
        if count <= TAIL_CAP {
            if count == TAIL_CAP {
                let root= memory::space_for(TAIL_CAP);
                unsafe {
                    use std::slice;
                    let mut root_contents = slice::from_raw_parts_mut(root.offset(1), TAIL_CAP);
                    root_contents.copy_from_slice(fields.inline_tail());
                }
                let tail = memory::space_for(TAIL_CAP);
                unsafe {
                    *tail.offset(1) = x;
                }
                let base_ptr = memory::space_for(6 /*field count*/);
                let dispatch_ptr = unsafe { base_ptr.offset(1) };
                Vector::store_trait_table(dispatch_ptr);
                let mut fields = VectorFields { dispatch: dispatch_ptr };
                fields.set_dispatch_offset(1);
                fields.set_non_count_bits(32);
                fields.set_count(TAIL_CAP + 1);
                fields.set_meta(Value::NIL);
                fields.set_hash(0);
                fields.set_root(root as u64);
                fields.set_tail(tail as u64);
                // TODO manage sharing
                return Value { handle: base_ptr as usize };
            } else {
                // TODO grow inline tail
            }
        } else {
            // TODO grow proper tree
        }




        if memory::is_shared(fields.capacity_ptr()) {
            unimplemented!()
        } else {
            //fields.set_count(fields.count() + 1);
            fields.set_hash(0);

        }
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
