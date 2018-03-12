#[cfg(target_pointer_width = "64")]
mod hello;


#[derive(Debug)]
struct Value {
    handle: usize,
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
