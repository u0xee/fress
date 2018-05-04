use Value;
use memory::unit::Unit;

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        let u = Unit::from(x).u();
        Value { handle: Unit::from((u & !0x0F) + 0x09) }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        let u: usize = self.handle.into();
        Unit::from(u & !0x0F).into()
    }
}
