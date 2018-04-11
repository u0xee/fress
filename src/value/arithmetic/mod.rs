
use std::ops::{Add};
use Value;

fn coerce_to_float(v: &Value) -> f64 {
    let c = Value { handle: v.handle };
    if v.is_float_point() {
        c.into()
    } else {
        let x: i64 = c.into();
        x as f64
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        if self.is_immediate_number() && rhs.is_immediate_number() {
            let float = self.is_float_point() || rhs.is_float_point();
            if float {
                let x = coerce_to_float(&self);
                let y = coerce_to_float(&rhs);
                Value::from(x + y)
            } else {
                let x: i64 = self.into();
                let y: i64 = rhs.into();
                Value::from(x + y)
            }
        } else { panic!("Add not implemented for non-immediate_number types.") }
    }
}
