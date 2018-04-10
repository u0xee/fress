use Value;

impl From<u64> for Value {
    fn from(x: u64) -> Self {
        Value { handle: ((x << 3) + 1) as usize }
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        (self.handle >> 3) as u64
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u64() {
        let y: Value = 17u64.into();
        let z: u64 = y.into();
        assert_eq!(17, z)
    }
}
