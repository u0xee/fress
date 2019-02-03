// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;

pub fn meta(prism: AnchoredLine) -> Unit {
    let guide = Guide::hydrate(prism);
    if guide.has_meta() {
        guide.meta_line()[0]
    } else {
        Handle::nil().unit()
    }
}

pub fn with_meta(prism: AnchoredLine, m: Unit) -> Unit {
    let guide = unaliased_root(Guide::hydrate(prism));
    if guide.has_meta() {
        guide.retire_meta();
        guide.meta_line().set(0, m);
        guide.segment().unit()
    } else {
        let cap = guide.segment().capacity();
        let mi = guide.meta_line().index;
        let s = Segment::new(cap + 1);
        guide.segment().at(0..mi).to(s);
        guide.segment().at(mi..cap).to_offset(s, mi + 1);
        s.set(mi, m);
        Segment::free(guide.segment());
        let mut g = guide;
        g.prism = guide.prism.with_seg(s);
        g.set_meta().reroot().store().segment().unit()
    }
}
