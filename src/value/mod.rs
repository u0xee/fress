/*
Methods on Value are the main library API:
- Static code dispatching based on union base, possibly to assembled trait object
- &Value into another Value (during its scope)
- Special high level operations like split

Types:
- boolean
- nil
- char
- string
- symbol
- keyword
- integral
- rational
- float point

- List
- Vector
- Map
- Set
- SortedMap
- SortedSet
*/

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub struct Value {
    pub handle: usize,
}

use std;
use dispatch::Dispatch;

impl Value {
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
}

impl Value {
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
    fn map_value(&self) {
        panic!("{} is NOT a MapValue", self.type_name())
    }
    fn set_value(&self) {
        panic!("{} is NOT a SetValue", self.type_name())
    }
    fn vector_value(&self) {
        panic!("{} is NOT a VectorValue", self.type_name())
    }
    fn list_value(&self) {
        panic!("{} is NOT a ListValue", self.type_name())
    }
    fn string_value(&self) {
        panic!("{} is NOT a StringValue", self.type_name())
    }
    fn symbol(&self) {
        panic!("{} is NOT a Symbol", self.type_name())
    }
    fn keyword(&self) {
        panic!("{} is NOT a Keyword", self.type_name())
    }
    fn integral(&self) {
        panic!("{} is NOT an Integral", self.type_name())
    }
    fn rational(&self) {
        panic!("{} is NOT a Rational", self.type_name())
    }
    fn float_point(&self) {
        panic!("{} is NOT a FloatPoint", self.type_name())
    }
}


// Data and vtable for trait
struct SeqValue {}
struct CollValue {}
struct AssociativeValue {}
struct SequentialValue {}
struct SortedValue {}
struct NumericValue {}

// Constructed via typecheck on Value
// Static dispatch to methods
struct MapValue {}
struct SortedMapValue {}
struct SetValue {}
struct SortedSetValue {}
struct VectorValue {}
struct ListValue {}
struct StringValue {}
struct Symbol {}
struct Keyword {}
struct Integral {}
struct Rational {}
struct FloatPoint {}
