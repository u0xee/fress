// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;

pub struct Anchor {
    pub unit: Unit,
}

impl Anchor {
    pub fn for_capacity(capacity: u32) -> Anchor {
        let bit_fields = (capacity << 16) | 1u32;
        Unit::from(bit_fields ).into()
    }

    pub fn capacity(&self) -> u32 {
        let c: u32 = self.unit.into();
        c >> 16
    }

    pub fn aliases(&self) -> u32 {
        let c: u16 = self.unit.into();
        c as u32
    }

    pub fn is_aliased(&self) -> bool {
        self.aliases() != 1
    }
}

impl From<Unit> for Anchor {
    fn from(u: Unit) -> Self {
        Anchor { unit: u }
    }
}
