// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

#[derive(Copy, Clone)]
pub struct Pop {
    pub child_turnout: u32,
    pub key_turnout: u32,
}

impl Pop {
    pub fn new() -> Pop {
        Pop { child_turnout: 0, key_turnout: 0 }
    }

    pub fn hydrate(x: u64) -> Pop {
        let mask = (1u64 << ARITY) - 1;
        Pop { child_turnout: (x & mask) as u32, key_turnout: ((x >> ARITY) & mask) as u32 }
    }

    pub fn store(&self) -> u64 {
        let mask = (1u64 << ARITY) - 1;
        let bottom = self.child_turnout as u64 & mask;
        let top = (self.key_turnout as u64 & mask) << ARITY;
        top | bottom
    }

    pub fn child_count(&self) -> u32 {
        self.child_turnout.count_ones()
    }

    pub fn key_count(&self) -> u32 {
        self.key_turnout.count_ones()
    }

    pub fn has_child(&self, hash_chunk: u32) -> bool {
        let test_bit = 1u32 << hash_chunk;
        (test_bit & self.child_turnout) != 0
    }

    pub fn has_key(&self, hash_chunk: u32) -> bool {
        let test_bit = 1u32 << hash_chunk;
        (test_bit & self.key_turnout) != 0
    }

    pub fn children_below(&self, hash_chunk: u32) -> u32 {
        let test_bit = 1u32 << hash_chunk;
        let mask = test_bit - 1;
        (self.child_turnout & mask).count_ones()
    }

    pub fn keys_below(&self, hash_chunk: u32) -> u32 {
        let test_bit = 1u32 << hash_chunk;
        let mask = test_bit - 1;
        (self.key_turnout & mask).count_ones()
    }

    pub fn unit(self) -> Unit {
        self.into()
    }

    pub fn flip_child(&self, hash_chunk: u32) -> Pop {
        let bit = 1u32 << hash_chunk;
        Pop { child_turnout: self.child_turnout ^ bit, key_turnout: self.key_turnout }
    }

    pub fn flip_key(&self, hash_chunk: u32) -> Pop {
        let bit = 1u32 << hash_chunk;
        Pop { child_turnout: self.child_turnout, key_turnout: self.key_turnout ^ bit }
    }
}

impl From<Unit> for Pop {
    fn from(u: Unit) -> Self {
        Pop::hydrate(u.into())
    }
}

impl Into<Unit> for Pop {
    fn into(self) -> Unit {
        Unit::from(self.store())
    }
}

use std::fmt;
impl fmt::Debug for Pop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@[")?;
        for i in 0..ARITY {
            if self.has_child(i) {
                write!(f, "C")?;
            } else if self.has_key(i) {
                write!(f, "K")?;
            } else {
                write!(f, "x")?;
            }
            let k = i + 1;
            if k % 4 == 0 && k != ARITY {
                write!(f, " ")?;
            }
        }
        write!(f, "]")
    }
}

