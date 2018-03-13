#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
struct Value {
    handle: usize,
}

impl Value {
    fn split(self) -> (Value, Value) {
        // TODO support non immediate values
        (Value {handle: self.handle}, Value {handle: self.handle})
    }

    fn is_immediate(&self) -> bool {
        self.handle & 0x01 == 0x01
    }

    fn is_not(&self) -> bool {
        self.handle & 0x0F == 0x07
    }

    fn is_so(&self) -> bool {
        !self.is_not()
    }

    // TODO associated constants may not be good. factory fn?
    const NIL: Value = Value {handle: 0x07};

    fn is_nil(&self) -> bool {
        self.handle == 0x07
    }

    const TRUE: Value = Value {handle: std::usize::MAX};

    fn is_true(&self) -> bool {
        self.handle == Value::TRUE.handle
    }

    const FALSE: Value = Value {handle: !0x08};

    fn is_false(&self) -> bool {
        self.handle == std::usize::MAX & !8
    }

    fn is_char(&self) -> bool {
        self.handle & 0x07 == 0x03
    }

    fn is_immediate_number(&self) -> bool {
        self.handle & 0x03 == 0x01
    }

    fn is_number(&self) -> bool {
        // TODO support boxed numbers
        self.is_immediate_number()
    }

    fn as_pointer(&self) -> *mut () {
        self.handle as *mut ()
    }

    fn as_i64(&self) -> i64 {
        (self.handle as i64) >> 3
    }
}


// https://github.com/rust-lang/rust/issues/27700#issuecomment-169014713
fn allocate(word_count: usize) -> *mut u64 {
    let mut v = Vec::with_capacity(word_count);
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

fn deallocate(ptr: *mut u64, count: usize) {
    unsafe {
        std::mem::drop(Vec::from_raw_parts(ptr, 0, count));
    }
}


struct MapHeader {
    count: usize,
    hasheq: u32,
    meta: Value,
    pop: u32,
    inode_pop: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocate_deallocate() {
        let x = allocate(34);
        deallocate(x, 34);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn print_values() {
        assert_eq!("(Value { handle: 7 }, Value { handle: 7 })",
                   format!("{:?}", (Value {handle: 7}.split())));
    }

    #[test]
    fn is_immediate() {
        assert!(Value {handle: 7}.is_immediate())
    }

    #[test]
    fn is_not() {
        assert!(Value::NIL.is_not() && Value::FALSE.is_not())
    }

    #[test]
    fn is_so() {
        assert!(Value {handle: 0}.is_so())
    }

    #[test]
    fn is_nil() {
        assert!(Value {handle: 7}.is_nil())
    }

    #[test]
    fn is_true() {
        assert!(Value {handle: !0}.is_true())
    }

    #[test]
    fn is_false() {
        assert!(Value {handle: !0 - 8}.is_false())
    }

    #[test]
    fn is_immediate_number() {
        assert!(Value {handle: 1}.is_immediate_number() &&
        Value {handle: 5}.is_immediate_number())
    }
}
