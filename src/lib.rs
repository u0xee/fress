//! A cohesive fressian library for rust

mod bit;
mod dispatch;
mod keyword;
mod list;
mod map;
mod memory;
mod method_union;
mod rational;
mod set;
mod sorted_map;
mod sorted_set;
mod string;
mod symbol;
mod value;
mod vector;

pub use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_integers() {
        let seven: Value = 7.into();
        let eight = 8.into();
        let ans: i32 = (seven + eight).into();
        assert_eq!(ans, 15)
    }

    #[test]
    fn add_floats() {
        let seven: Value = 7.25.into();
        let eight = 8.5.into();
        let ans: f64 = (seven + eight).into();
        assert_eq!(ans, 15.75)
    }

    #[test]
    fn conj_vector() {
        use value::distinguish::VectorValue;
        let mut v = VectorValue::new();

        for x in 1..5 {
            v = v.conj(x.into());
        }

        let s = String::from("[1 2 3 4 5]");
        assert_eq!(v.edn(), s)
    }
}
