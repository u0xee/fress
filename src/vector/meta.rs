// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn meta(prism: Line) -> Unit {
    let guide: Guide = prism[1].into();
    if guide.has_meta() {
        let anchor_gap = guide.prism_to_anchor_gap();
        let root_gap = guide.guide_to_root_gap();
        let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
        segment[3 /*anchor, prism, guide*/ + anchor_gap + guide.meta_gap()]
    } else {
        Value::NIL
    }
}

pub fn with_meta(prism: Line, m: Unit) -> Unit {
    let guide: Guide = prism[1].into();
    let anchor_gap = guide.prism_to_anchor_gap();
    let root_gap = guide.guide_to_root_gap();
    let count = guide.count();
    let tailoff = (count - 1) & !MASK;
    let root_count = if count <= TAIL_CAP { count } else { root_content_count(tailoff) };

    let segment: Segment = prism.offset(-((anchor_gap + 1) as isize)).into();
    let mut s = if segment.is_aliased() {
        unalias_root(segment, anchor_gap, root_gap, root_count, guide)
    } else { segment };

    let meta_idx = 3 /*anchor, prism, guide*/ + anchor_gap + guide.meta_gap();
    if guide.has_meta() {
        ValueUnit::from(s[meta_idx]).retire();
        s[meta_idx] = m;
        Unit::from(s)
    } else {
        let used_units = anchor_gap + root_gap + root_count + 3 /*anchor, prism, guide*/;
        let cap = s.capacity();
        let mut h = if used_units == cap {
            let mut h = Segment::with_capacity(used_units + 1);
            for i in 1..used_units {
                h[i] = s[i];
            }
            Segment::free(s);
            h
        } else { s };
        for i in (meta_idx..used_units).rev() {
            h[i + 1] = h[i];
        }
        h[meta_idx] = m;
        h[2 + anchor_gap] = guide.with_meta().into();
        Unit::from(h)
    }
}

