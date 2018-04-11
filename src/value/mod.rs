mod aggregate;
mod arithmetic;
pub mod distinguish;
mod immediate;

/*
Value is the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- &Value into another Value (during its scope)
- Special high level operations like split
*/

use memory::unit::Unit;
use dispatch::{Distributor, as_dispatch_obj};

#[derive(Debug)]
pub struct Value {
    pub handle: Unit,
}

use std;
use dispatch::Dispatch;

impl Value {
    pub const NIL: Value = Value { handle: Unit { word: 0x07 } };
    pub const TRUE: Value = Value { handle: Unit { word: std::usize::MAX } };
    pub const FALSE: Value = Value { handle: Unit { word: !0x08usize } };

    pub fn type_name(&self) -> String {
        if self.is_immediate() {
            "Immediate Value".to_string()
        } else {
            unsafe {
                let d: *const Distributor = self.handle.into();
                let distributor_offset = 1;
                let o = as_dispatch_obj(d.offset(distributor_offset));
                o.type_name()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passes() {
        assert!(true)
    }
    /*
    #[test]
    fn testbed() {
        let x = Value { handle: 7 };

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

    #[test]
    fn from_u64() {
        let x: u64 = 17;
        let y: Value = x.into();
        let z: u64 = y.into();
        assert_eq!(x, z)
    }
    */
}
