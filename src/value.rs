#[derive(Debug)]
pub struct Value {
    pub(crate) handle: usize,
}

impl Value {
    pub fn split(self) -> (Value, Value) {
        (Value {handle: self.handle}, Value {handle: self.handle})
    }
}
