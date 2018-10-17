use Value;
use memory::unit::Unit;

impl From<u64> for Value {
    fn from(x: u64) -> Self {
        Value { handle: Unit::from((x << 4) + 1) }
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        let u: u64 = self.handle.into();
        u >> 4
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        Value { handle: Unit::from((x << 4) + 1) }
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        let i: i64 = self.handle.into();
        i >> 4
    }
}

impl From<u32> for Value {
    fn from(x: u32) -> Self {
        Value { handle: Unit::from((x << 4) + 1) }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        let u: u32 = self.handle.into();
        u >> 4
    }
}

impl From<i32> for Value {
    fn from(x: i32) -> Self {
        Value { handle: Unit::from((x << 4) + 1) }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        let i: i32 = self.handle.into();
        i >> 4
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u64() {
        let y: Value = 17u64.into();
        let z: u64 = y.into();
        assert_eq!(17u64, z)
    }
}
