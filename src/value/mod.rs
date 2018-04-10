mod distinguish;
mod immediate;

/*
Value is the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- &Value into another Value (during its scope)
- Special high level operations like split
*/

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub struct Value {
    pub handle: usize,
}

use std;
use dispatch::Dispatch;

impl From<u64> for Value {
    fn from(x: u64) -> Self {
        Value { handle: x as usize }
    }
}

impl Value {
    //! handle bit patterns:
    //!
    //! !(handle | 0x08) => 0, boolean
    //! 0xFFFFFFFFFFFFFFFF true
    //! 0xFFFFFFFFFFFFFFF7 false
    //!
    //! End in 0b0111, logically negative
    //! 0x0000000000000007 nil
    //! 0xFFFFFFFFFFFFFFF7 false
    //!
    //! End in 0b011
    //! 0xXXXXXXXX00000003 char
    //! 0xXXXXXXXXXXXXXXLB string, L holds count
    //!
    //! End in 0b001
    //! 0xXXXXXXXXXXXXXXX1 integral
    //! 0xXXXXXXXXXXXXXXX9 FloatPoint
    //!
    //! Even handles (rightmost bit of 0) are pointers.
    //! They point to segments that have a distributor as the first unit.
    //!

    pub fn split(self) -> (Value, Value) {
        // TODO support non immediate values

        (Value {handle: self.handle}, Value {handle: self.handle})
    }

    pub fn is_immediate(&self) -> bool {
        self.handle & 0x01 == 0x01
    }

    pub fn is_not(&self) -> bool {
        self.handle & 0x0F == 0x07
    }

    pub fn is_so(&self) -> bool {
        !self.is_not()
    }

    // TODO associated constants may not be good. factory fn?
    pub const NIL: Value = Value {handle: 0x07};

    pub fn is_nil(&self) -> bool {
        self.handle == 0x07
    }

    pub const TRUE: Value = Value {handle: std::usize::MAX};

    pub fn is_true(&self) -> bool {
        self.handle == Value::TRUE.handle
    }

    pub const FALSE: Value = Value {handle: !0x08};

    pub fn is_false(&self) -> bool {
        self.handle == std::usize::MAX & !8
    }

    pub fn is_char(&self) -> bool {
        self.handle & 0x07 == 0x03
    }

    pub fn is_string(&self) -> bool {
        // TODO separate char immediate and string
        self.handle & 0x07 == 0x03
    }

    pub fn is_immediate_number(&self) -> bool {
        self.handle & 0x03 == 0x01
    }

    pub fn is_number(&self) -> bool {
        // TODO support boxed numbers
        self.is_immediate_number()
    }

    pub fn as_pointer(&self) -> *mut () {
        self.handle as *mut ()
    }

    pub fn as_i64(&self) -> i64 {
        (self.handle as i64) >> 3
    }

    pub fn type_name(&self) -> String {
        if self.is_immediate() {
            "Immediate Value".to_string()
        } else {
            unsafe {
                let table_ptr = (self.handle as *const u64).offset(1);
                let t_object: [u64; 2] = [table_ptr as u64, *table_ptr];
                use std::mem::transmute;
                let dispatch = transmute::<[u64;2], &Dispatch>(t_object);
                let s = dispatch.type_name();
                use std::mem::forget;
                //forget(dispatch);
                s
            }
        }
    }
}

