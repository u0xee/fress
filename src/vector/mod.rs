// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory;
use bit::{bottom_32, top_32, top_16, MASK_32, MASK_16, top_byte, with_top_byte, second_top_byte, clear_top, clear_bottom, splice, with_second_top_byte};
use dispatch::Dispatch;
use Value;

pub mod tree;
pub mod memory;

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
            clear_top(field, self.non_count_bits()) as u32
        }
    }

    fn set_count(&mut self, c: u32) {
        unsafe {
            let field = *self.dispatch.offset(FIELD_COUNT);
            *self.dispatch.offset(FIELD_COUNT) =
                splice(field, c as u64, self.non_count_bits());
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
            (field >> 16) as u32
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
            (field << 48 >> 56) as u8
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
            let first =
                self.dispatch.offset(FIELD_COUNT + self.inline_tail_has_meta() as isize + 1);
            slice::from_raw_parts(first, self.count() as usize)
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

use dispatch::{Identification, Distinguish, AggregateAbstractions, StreamlinedMethods};

impl Identification for Vector {
    fn type_name(&self) -> String {
        "Vector".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Vector {
    fn hash(&self) -> u32 {
        unimplemented!()
    }

    fn eq(&self, other: &Dispatch) -> bool {
        unimplemented!()
    }

    fn cmp(&self, other: &Dispatch) -> Ordering {
        unimplemented!()
    }
}

impl AggregateAbstractions for Vector {
}

impl StreamlinedMethods for Vector {
}

impl Dispatch for Vector {
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
    /*bits in a u32*/ 32 - x.leading_zeros() as u8
}

fn digit_count(x: u32) -> u8 {
    ((significant_bits(x) as u32 + BITS - 1) as u32 / BITS) as u8
}

fn digit(x: u32, idx: u8) -> u8 {
    (x >> (idx as u32 * BITS)) as u8
}

fn digit_iter(x: u32, digits: u8) {
    // Digit iterator struct
}

use dispatch::{Distributor, distributor};

/*
impl Vector {
    fn store_trait_table(location: *mut u64) {
        unsafe {
            let d = distributor::<Vector>();
            *location = d.opaque_method_table_ptr.into()
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
        Value { handle: Unit::from(base_ptr) }
    }

    fn conj(&mut self, x: Value) -> Value {
        let mut fields = VectorFields { dispatch: &mut self.base as *mut u64 };
        let count = fields.count();
        if count <= TAIL_CAP {
            if count == TAIL_CAP {
                let root= memory::space_for(TAIL_CAP as usize);
                unsafe {
                    use std::slice;
                    let mut root_contents =
                        slice::from_raw_parts_mut(root.offset(1),
                                                  TAIL_CAP as usize);
                    root_contents.copy_from_slice(fields.inline_tail());
                }
                let tail = memory::space_for(TAIL_CAP as usize);
                unsafe {
                    *tail.offset(1) = x.handle as u64;
                }
                let base_ptr = memory::space_for(6 /*field count*/);
                let dispatch_ptr = unsafe { base_ptr.offset(1) };
                Vector::store_trait_table(dispatch_ptr);
                let mut fields = VectorFields { dispatch: dispatch_ptr };
                fields.set_dispatch_offset(1);
                fields.set_non_count_bits(32);
                fields.set_count(TAIL_CAP + 1);
                fields.set_meta(Value::NIL.as_i64() as u64);
                fields.set_hash(0);
                fields.set_root(root as u64);
                fields.set_tail(tail as u64);
                // TODO manage sharing
                return Value { handle: Unit::from(base_ptr) };
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
            unimplemented!()
        }
    }
}
*/

/*
mod path_copy {
    use memory;

    pub struct Tree {
        base: *const u64,
        header_word_count: u16,
        pointer_count: u16,
        index: u16,
        shared: bool,
    }

    fn not_shared_count(path: &[Tree]) -> u8 {
        for (index, tree) in path.iter().enumerate() {
            if memory::is_shared(tree.base) {
                return index as u8
            }
        }
        path.len()
    }

    fn is_three_quarters(filled: u16, cap: u16) -> bool {
        let line = (3 * cap) / 4;
        filled >= line
    }

    pub fn set(path: &mut[Tree], v: u64, shared_root: bool) -> *mut u64 {
        let (exclusive, shared) =
            path.split_at_mut(if shared_root { 0 } else { not_shared_count(path) });
        for tree in exclusive {
            tree.shared = false;
        }
        for tree in shared {
            tree.shared = true;
        }
        let (leaf, but_leaf) = path.split_last().unwrap();
        let set_leaf = settable_leaf(leaf);

        unsafe { *set_leaf.offset(leaf.index as isize) = v; }

        let root: *mut u64 = but_leaf.iter().rev().fold(set_leaf, thread_through_trees);
        root
    }

    fn settable_leaf(leaf: &Tree) -> *mut u64 {
        let filled_count = leaf.header_word_count + leaf.pointer_count;
        let is_append = leaf.index == filled_count;
        assert!(leaf.index <= filled_count);

        let leaf_cap = memory::capacity_of(leaf.base);
        if leaf.shared {
            let double_now = leaf_cap < super::TAIL_CAP &&
                is_three_quarters(filled_count, leaf_cap);
            let copy_capacity = leaf_cap << if double_now { 1 } else { 0 };
            let leaf_copy = memory::copy_of(leaf.base, filled_count, copy_capacity);
            for i in leaf.header_word_count..filled_count {
                if i != leaf.index {
                    unsafe {
                        memory::register_interest(leaf.base.offset(i));
                    }
                }
            }
            leaf_copy
        } else { // not shared
            if is_append && !(leaf.index < leaf_cap) {
                memory::copy_of(leaf.base, filled_count - 1, leaf_cap << 1)
            } else {
                leaf.base
            }
        }
    }

    fn thread_through_trees(prev_tree: *mut u64, tree: &Tree) -> *mut u64 {
        if tree.shared {
            let tree_filled = tree.header_word_count + tree.pointer_count;
            let tree_copy = memory::copy_of(tree.base, tree_filled - 1,
                                            memory::capacity_of(tree.base) << 1);
            for i in tree.header_word_count..tree_filled {
                if i != tree.index {
                    unsafe {
                        memory::register_interest(tree.base.offset(i));
                    }
                }
            }
            unsafe {
                *tree_copy.offset(tree.index) = prev_tree;
            }
            tree_copy
        } else {
            unsafe {
                memory::deregister_interest(tree.base.offset(tree.index));
                *tree.base.offset(tree.index) = prev_tree;
            }
            tree.base as *mut u64
        }
    }

    pub fn unset(path: &mut[Tree], shared_root: bool) -> *mut u64 {
        unimplemented!()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

}