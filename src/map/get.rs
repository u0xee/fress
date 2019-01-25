// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn get(prism: AnchoredLine, k: Unit, hash: u32, has_vals: u32) -> Option<Unit> {
    let guide = Guide::hydrate(prism);


    let (base, pop, hash_stack, chunks, child_count) = {
        let mut pop: Pop = prism[2 + root_gap as usize].into();
        let mut base: Line = prism.offset((2 + root_gap) as isize);
        let mut child_count: u32 = 0;
        let mut hash_stack = hash;
        let mut chunks = MAX_LEVELS;
        loop {
            match pop.child_idx(hash_stack & MASK) {
                Ok(idx) => {
                    let i = idx * 2;
                    pop = base[1 + i as usize].into();
                    base = base[1 + i as usize + 1].into();
                    hash_stack = hash_stack >> BITS;
                    chunks = chunks - 1;
                },
                Err(c_count) => {
                    child_count = c_count;
                    break;
                },
            }
        }
        (base, pop, hash_stack, chunks, child_count)
    };

    match pop.key_idx(hash_stack & MASK) {
        Ok(idx) => {
            let i = child_count * 2 + idx * 2;
            if (chunks == 1) && pop.child_idx(hash_stack & MASK).is_ok() {
                let collision_count: u32 = base[1 + i as usize].into();
                let collision: Segment = base[1 + i as usize + 1].into();
                for j in 0..collision_count {
                    let maybe_k = collision[1 + j * 2];
                    if ValueUnit::from(k).eq(maybe_k) {
                        return Some(collision[1 + j * 2 + 1]);
                    }
                }
                None
            } else {
                let maybe_k = base[1 + i as usize];
                if ValueUnit::from(k).eq(maybe_k) {
                    Some(base[1 + i as usize + 1])
                } else {
                    None
                }
            }
        },
        Err(k_count) => {
            None
        },
    }
}
