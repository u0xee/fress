use super::unit::Unit;

pub struct Anchor {
    unit: Unit,
}

pub struct AnchorLine {
    line: *const Anchor,
}
