// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use std::fmt;
use memory::*;
use dispatch::*;
use value::Value;
use handle::Handle;

pub static KEYWORD_SENTINEL: u8 = 0;

pub struct Keyword {
    prism: Unit,
}

impl Keyword {
    pub fn new() -> Unit {
        // take string
        unimplemented!()
    }

    pub fn new_from_str(source: &str) -> Unit {
        // into string
        unimplemented!()
    }
}

impl Dispatch for Keyword {
    fn tear_down(&self, prism: AnchoredLine) {
        // segment has 0 aliases
        //Segment::free(prism.segment())
        unimplemented!()
    }
}

impl Identification for Keyword {
    fn type_name(&self) -> &'static str {
        "Keyword"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& KEYWORD_SENTINEL) as *const u8
    }
}

use std::cmp::Ordering;
impl Distinguish for Keyword {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        unimplemented!()
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        unimplemented!()
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Option<Ordering> {
        unimplemented!()
    }
}
impl Aggregate for Keyword { }
impl Associative for Keyword { }
impl Sequential for Keyword { }
impl Reversible for Keyword { }
impl Sorted for Keyword { }
impl Notation for Keyword {
    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
impl Numeral for Keyword { }

#[cfg(test)]
mod tests {
    use super::*;
}

