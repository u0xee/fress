use std::mem;

mod unit;
mod anchor;
mod segment;


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

pub const CACHE_LINE: usize = 8;

pub fn space(capacity: usize) -> *mut u64 {
    let mut v: Vec<u64> = Vec::with_capacity(capacity);
    let size_in_bytes: u64 = (capacity as u64) << 3; // times 8
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = size_in_bytes;
    }
    mem::forget(v);
    ptr
}

pub fn space_for(capacity: usize) -> *mut u64 {
    space(capacity + 1)
}

pub fn capacity_of(ptr: *const u64) -> u64 {
    unsafe {
        // TODO long loads atomic on x86, I think. Verify
        let size_in_bytes = *ptr;
        if size_in_bytes & 0x01 == 0x00 {
            size_in_bytes >> 3 // divide by 8
        } else {
            // Clear the bottom bit of the marked pointer
            let atomic_count_ptr: *const usize = (size_in_bytes & !1u64) as *const usize;
            let size_in_bytes = *(atomic_count_ptr.offset(1));
            size_in_bytes as u64 >> 3
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

fn free_memory(ptr: *mut u64, capacity: usize) {
    unsafe {
        let v: Vec<u64> = Vec::from_raw_parts(ptr, 0, capacity);
        mem::drop(v);
    }
}

pub fn copy(from: *const u64, to: *mut u64, word_count: usize) {
    use std::slice::{from_raw_parts, from_raw_parts_mut};
    unsafe {
        from_raw_parts_mut(to, word_count).copy_from_slice(from_raw_parts(from, word_count))
    }
}

pub fn copy_of(source: *const u64, copy_word_count: usize, capacity: usize) -> *mut u64 {
    let c = space(capacity);
    unsafe {
        copy(source.offset(1), c.offset(1), copy_word_count);
    }
    c
}


