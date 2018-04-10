use super::unit::Unit;
use memory;

pub struct Anchor {
    pub unit: Unit,
}

impl Anchor {
    pub fn for_capacity(capacity: usize) -> Anchor {
        let ensure_odd = (capacity << 1) + 1;
        // ensure_odd is distinct from a pointer,
        // by virtue of being odd
        Unit::from(ensure_odd).into()
    }

    pub fn capacity(&self) -> usize {
        let c: usize = self.unit.into();
        c >> 1
    }
}

pub struct AnchorLine {
    pub line: *const Anchor,
}

impl AnchorLine {
    pub fn set_anchor(&self, a: Anchor) {
        memory::set(Unit::from(self.line).into(), a.into())
    }

    pub fn get_anchor(&self) -> Anchor {
        let anchor_or_ptr =
            memory::get(Unit::from(self.line).into());
        if anchor_or_ptr.is_even() {
            memory::get(anchor_or_ptr.into()).into()
        } else {
            anchor_or_ptr.into()
        }
    }
}
