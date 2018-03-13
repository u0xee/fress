#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub struct Value {
    pub(crate) handle: usize,
}

impl Value {
    pub fn split(self) -> (Value, Value) {
        (Value {handle: self.handle}, Value {handle: self.handle})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
