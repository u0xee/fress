// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn nth(prism: AnchoredLine, idx: u32) -> Unit {
    let guide = Guide::hydrate(prism);
    if idx >= guide.count {
        panic!("Index out of bounds: {} in vector of count {}", idx, guide.count);
    }
    if guide.count <= TAIL_CAP {
        guide.root[idx as i32]
    } else {
        nth_tailed(guide, idx)
    }
}

fn nth_tailed(guide: Guide, idx: u32) -> Unit {
    let tailoff = tailoff(guide.count);
    if idx >= tailoff {
        guide.root[-1].segment()[idx - tailoff]
    } else {
        let digit_count = digit_count(tailoff - 1);
        let mut shift = digit_count * BITS;
        let mut curr = {
            shift -= BITS;
            guide.root.offset(last_digit(idx >> shift) as i32)
        };
        for _ in 0..(digit_count - 1) {
            let digit = {
                shift -= BITS;
                last_digit(idx >> shift)
            };
            curr = curr[0].segment().line_at(digit);
        }
        assert_eq!(shift, 0);
        curr[0]
    }
}
