// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

#[derive(Copy, Clone)]
pub struct Pop {
    pub population: Unit,
}

impl Pop {
    pub fn new() -> Pop {
        Pop { population: Unit::from(0) }
    }

    pub fn child_pop(&self) -> u32 {
        let x: u64 = self.population.into();
        let mask = (1u64 << ARITY) - 1;
        (x & mask) as u32
    }

    pub fn child_pop_count(&self) -> u32 {
        self.child_pop().count_ones()
    }

    pub fn key_pop(&self) -> u32 {
        let x: u64 = self.population.into();
        let y = x >> ARITY;
        let mask = (1u64 << ARITY) - 1;
        (y & mask) as u32
    }

    pub fn key_pop_count(&self) -> u32 {
        self.key_pop().count_ones()
    }

    pub fn from_pops(child_pop: u32, key_pop: u32) -> Pop {
        let mask = (1u64 << ARITY) - 1;
        let top = ((key_pop as u64) & mask) << ARITY;
        let bottom = (child_pop as u64) & mask;
        let p = top | bottom;
        Pop { population: Unit::from(p) }
    }

    fn index_in_pop(pop: u32, hash_chunk: u32) -> Result<u32, u32> {
        let test_bit = 1u32 << hash_chunk;
        if (test_bit & pop) != 0 {
            let mask = test_bit - 1;
            let members_before_target = (pop & mask).count_ones();
            Ok(members_before_target)
        } else {
            Err(pop.count_ones())
        }
    }

    pub fn child_idx(&self, hash_chunk: u32) -> Result<u32, u32> {
        let c = self.child_pop();
        Pop::index_in_pop(c, hash_chunk)
    }

    pub fn key_idx(&self, hash_chunk: u32) -> Result<u32, u32> {
        let k = self.key_pop();
        Pop::index_in_pop(k, hash_chunk)
    }

    pub fn any_idx(&self, hash_chunk: u32) -> bool {
        let test_bit = 1u32 << hash_chunk;
        let combined_pop = self.child_pop() | self.key_pop();
        (test_bit & combined_pop) != 0
    }
}

impl From<Unit> for Pop {
    fn from(u: Unit) -> Self {
        Pop { population: u }
    }
}

impl Into<Unit> for Pop {
    fn into(self) -> Unit {
        self.population
    }
}
