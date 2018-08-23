// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use memory::line::Line;
use memory::segment::Segment;
use super::*;
use std::mem::transmute;
use std::fmt;

pub fn prism<T: Dispatch>() -> Unit {
    unsafe {
        let as_ref = &*(0 as *const T);
        let as_ob = as_ref as &Dispatch;
        let null_and_table = transmute::<&Dispatch, [Unit; 2]>(as_ob);
        assert_eq!(Unit::from(0), null_and_table[0]);
        null_and_table[1]
    }
}

fn as_dispatch<'a>(line_to_prism: &'a Line) -> &'a Dispatch {
    let prism = line_to_prism[0];
    let ptr_and_table: [Unit; 2] = [Unit::from(line_to_prism.line), prism];
    unsafe {
        transmute::<[Unit; 2], &Dispatch>(ptr_and_table)
    }
}

impl Dispatch for Line {
    fn tear_down(&self) {
        as_dispatch(self).tear_down()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        as_dispatch(self).fmt(f)
    }
}

impl Identification for Line {
    fn type_name(&self) -> String {
        as_dispatch(self).type_name()
    }

    fn type_sentinel(&self) -> *const u8 {
        as_dispatch(self).type_sentinel()
    }
}

impl Distinguish for Line {
    fn hash(&self) -> u32 {
        as_dispatch(self).hash()
    }

    fn eq(&self, other: Unit) -> bool {
        as_dispatch(self).eq(other)
    }

    fn cmp(&self, other: Unit) -> Ordering {
        as_dispatch(self).cmp(other)
    }
}

impl Aggregate for Line {
    fn count(&self) -> u32 {
        as_dispatch(self).count()
    }

    fn empty(&self) -> Unit {
        as_dispatch(self).empty()
    }

    fn conj(&self, x: Unit) -> Unit {
        as_dispatch(self).conj(x)
    }

    fn meta(&self) -> Unit {
        as_dispatch(self).meta()
    }

    fn with_meta(&self, m: Unit) -> Unit {
        as_dispatch(self).with_meta(m)
    }

    fn peek(&self) -> Unit {
        as_dispatch(self).peek()
    }

    fn pop(&self) -> Unit {
        as_dispatch(self).pop()
    }

    fn get(&self, k: Unit) -> Unit {
        as_dispatch(self).get(k)
    }
}

impl Sequential for Line {
    fn nth(&self, idx: u32) -> Unit {
        as_dispatch(self).nth(idx)
    }
}

impl Associative for Line {
    fn contains(&self, x: Unit) -> bool {
        as_dispatch(self).contains(x)
    }

    fn assoc(&self, k: Unit, v: Unit) -> Unit {
        as_dispatch(self).assoc(k, v)
    }

    fn dissoc(&self, k: Unit) -> Unit {
        as_dispatch(self).dissoc(k)
    }
}

impl Reversible for Line {
    fn reverse(&self) -> Unit {
        as_dispatch(self).reverse()
    }
}

impl Sorted for Line {
    fn subrange(&self, start: Unit, end: Unit) -> Unit {
        as_dispatch(self).subrange(start, end)
    }
}

impl Named for Line {
    fn name(&self) -> Unit {
        as_dispatch(self).name()
    }

    fn namespace(&self) -> Unit {
        as_dispatch(self).namespace()
    }
}

// Teagano

impl Dispatch for Segment {
    fn tear_down(&self) {
        self.line.offset(1).tear_down()
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.line.offset(1).fmt(f)
    }
}

impl Identification for Segment {
    fn type_name(&self) -> String {
        self.line.offset(1).type_name()
    }

    fn type_sentinel(&self) -> *const u8 {
        self.line.offset(1).type_sentinel()
    }
}

impl Distinguish for Segment {
    fn hash(&self) -> u32 {
        self.line.offset(1).hash()
    }

    fn eq(&self, other: Unit) -> bool {
        self.line.offset(1).eq(other)
    }

    fn cmp(&self, other: Unit) -> Ordering {
        self.line.offset(1).cmp(other)
    }
}

impl Aggregate for Segment {
    fn count(&self) -> u32 {
        self.line.offset(1).count()
    }

    fn empty(&self) -> Unit {
        self.line.offset(1).empty()
    }

    fn conj(&self, x: Unit) -> Unit {
        self.line.offset(1).conj(x)
    }

    fn meta(&self) -> Unit {
        self.line.offset(1).meta()
    }

    fn with_meta(&self, m: Unit) -> Unit {
        self.line.offset(1).with_meta(m)
    }

    fn peek(&self) -> Unit {
        self.line.offset(1).peek()
    }

    fn pop(&self) -> Unit {
        self.line.offset(1).pop()
    }

    fn get(&self, k: Unit) -> Unit {
        self.line.offset(1).get(k)
    }
}

impl Sequential for Segment {
    fn nth(&self, idx: u32) -> Unit {
        self.line.offset(1).nth(idx)
    }
}

impl Associative for Segment {
    fn contains(&self, x: Unit) -> bool {
        self.line.offset(1).contains(x)
    }

    fn assoc(&self, k: Unit, v: Unit) -> Unit {
        self.line.offset(1).assoc(k, v)
    }

    fn dissoc(&self, k: Unit) -> Unit {
        self.line.offset(1).dissoc(k)
    }
}

impl Reversible for Segment {
    fn reverse(&self) -> Unit {
        self.line.offset(1).reverse()
    }
}

impl Sorted for Segment {
    fn subrange(&self, start: Unit, end: Unit) -> Unit {
        self.line.offset(1).subrange(start, end)
    }
}

impl Named for Segment {
    fn name(&self) -> Unit {
        self.line.offset(1).name()
    }

    fn namespace(&self) -> Unit {
        self.line.offset(1).namespace()
    }
}

