// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::*;
use map;
use handle;
use handle::Handle;
use vector::guide::Guide;

pub static SET_SENTINEL: u8 = 0;

pub struct Set {
    prism: Unit,
}

impl Set {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(3 + map::size(1));
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Set>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g
        };
        guide.root.set(-1, map::pop::Pop::new().unit());
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Set::new().handle().value()
    }
}

impl Dispatch for Set {
    fn tear_down(&self, prism: AnchoredLine) {
        map::tear_down::tear_down(prism, 0)
    }
}

impl Identification for Set {
    fn type_name(&self) -> String {
        "Set".to_string()
    }

    fn type_sentinel(&self) -> *const u8 {
        (& SET_SENTINEL) as *const u8
    }
}

impl Distinguish for Set {}

impl Aggregate for Set {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }

    fn empty(&self, prism: AnchoredLine) -> Unit {
        Set::new()
    }

    fn get(&self, prism: AnchoredLine, k: Unit) -> *const Unit {
        let h = k.handle().hash();
        if let Some(key_line) = map::get::get(prism, k, h, 0) {
            key_line.line().star()
        } else {
            (& handle::STATIC_NIL) as *const Unit
        }
    }

    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        let k = x;
        let h = k.handle().hash();
        let (g, key_slot) = map::assoc::assoc(prism, k, h, 0);
        match key_slot {
            Ok(new_slot) => {
                new_slot.set(0, k);
                g.inc_count().store().segment().unit()
            },
            Err(old_slot) => {
                k.handle().retire();
                g.store().segment().unit()
            },
        }
    }
}

impl Sequential for Set {}

impl Associative for Set {
    fn contains(&self, prism: AnchoredLine, k: Unit) -> bool {
        let h = k.handle().hash();
        map::get::get(prism, k, h, 0).is_some()
    }

    fn dissoc(&self, prism: AnchoredLine, k: Unit) -> Unit {
        let h = k.handle().hash();
        let g = map::dissoc::dissoc(prism, k, h, 0);
        g.segment().unit()
    }
}

impl Reversible for Set {}
impl Sorted for Set {}
impl Notation for Set {}


#[cfg(test)]
mod tests {
    use super::*;
}
