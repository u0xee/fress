use value::Value;

impl Value {
    pub fn map_value(&self) {
        panic!("{} is NOT a MapValue", self.type_name())
    }

    pub fn set_value(&self) {
        panic!("{} is NOT a SetValue", self.type_name())
    }

    pub fn vector_value(&self) {
        panic!("{} is NOT a VectorValue", self.type_name())
    }

    pub fn list_value(&self) {
        panic!("{} is NOT a ListValue", self.type_name())
    }

    pub fn string_value(&self) {
        panic!("{} is NOT a StringValue", self.type_name())
    }

    pub fn symbol(&self) {
        panic!("{} is NOT a Symbol", self.type_name())
    }

    pub fn keyword(&self) {
        panic!("{} is NOT a Keyword", self.type_name())
    }

    pub fn integral(&self) {
        panic!("{} is NOT an Integral", self.type_name())
    }

    pub fn rational(&self) {
        panic!("{} is NOT a Rational", self.type_name())
    }

    pub fn float_point(&self) {
        panic!("{} is NOT a FloatPoint", self.type_name())
    }
}


// Constructed via typecheck on Value
// Static dispatch to methods
struct MapValue {}
struct SortedMapValue {}
struct SetValue {}
struct SortedSetValue {}
struct VectorValue {}
struct ListValue {}
struct StringValue {}
struct Boolean {}
struct Symbol {}
struct Keyword {}
struct Integral {}
struct Rational {}
struct FloatPoint {}

// Data and vtable for trait
struct SeqValue {}
struct CollValue {}
struct AssociativeValue {}
struct SequentialValue {}
struct SortedValue {}
struct NumericValue {}
