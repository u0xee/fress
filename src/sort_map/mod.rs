// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Sorted balanced tree, supporting maps and sets.

use memory::*;
use dispatch::*;
use value::*;
use vector::guide::Guide;

pub const BITS: u32 = 4; // one of 4 or 5
pub const ARITY: u32 = 1 << BITS;
pub const MASK: u32 = ARITY - 1;

#[cfg(test)]
mod tests {
    use super::*;
}
