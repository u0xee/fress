// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::unit::Unit;

pub const ALIAS_BITS: u32 = if cfg!(target_pointer_width = "32") { 20 } else { 30 };
pub const ALIAS_MASK: u32 = (1 << ALIAS_BITS) - 1;

#[derive(Copy, Clone)]
pub struct Anchor {
    pub unit: Unit,
}

impl fmt::Debug for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Anchor {{ capacity: {}, aliases: {} }}",
               self.capacity(), self.aliases())
    }
}

impl Anchor {
    pub fn max_capacity() -> u32 {
        let cap_width = Unit::width() - ALIAS_BITS;
        use std::cmp::min;
        min((1u64 << cap_width) - 1, u32::MAX as u64) as u32
    }
    pub fn for_capacity(capacity: u32) -> Anchor {
        if capacity > Anchor::max_capacity() {
            panic!("Anchor capacity not representable: {}",
                   capacity);
        }
        let bit_fields = ((capacity as usize) << ALIAS_BITS) | 1;
        Unit::from(bit_fields).into()
    }
    pub fn capacity(&self) -> u32 {
        let c = self.unit.u() >> ALIAS_BITS;
        c as u32
    }
    pub fn aliases(&self) -> u32 {
        let c = self.unit.u() & ALIAS_MASK as usize;
        c as u32
    }
    pub fn is_aliased(&self) -> bool { self.aliases() != 1 }
    pub fn is_max_aliased(&self) -> bool {
        self.aliases() == ALIAS_MASK
    }
    pub fn aliased(&self) -> Anchor {
        if self.is_max_aliased() {
            panic!("Overflow of alias count!");
        }
        let x: usize = self.unit.into();
        Anchor { unit: Unit::from(x + 1) }
    }
    pub fn unaliased(&self) -> Anchor {
        if self.aliases() == 0 {
            panic!("Underflow of alias count!");
        }
        let x: usize = self.unit.into();
        Anchor { unit: Unit::from(x - 1) }
    }
    pub fn unit(&self) -> Unit { self.unit }
}

impl From<Unit> for Anchor { fn from(u: Unit) -> Self { Anchor { unit: u } } }
impl Into<Unit> for Anchor { fn into(self) -> Unit { self.unit } }

