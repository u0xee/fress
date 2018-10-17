// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::mem;

pub mod unit;
pub mod line;
pub mod segment;
pub mod segmen; //temp
pub mod anchor;
pub use self::unit::Unit;
pub use self::line::Line;
pub use self::segment::Segment;
pub use self::segment::AnchoredLine;
pub use self::anchor::Anchor;

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


#[cfg(test)]
mod test {
    use super::*;

}
