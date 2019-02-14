// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::value::Value;
use fress::handle::Handle;
use fress::integral::Integral;
use fress::transducer;
use fress::transducer::{Process, test_me};
use fress::value::{new_vector, new_list, new_map, new_set};

pub struct Printer {}
impl Process for Printer {
    fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
        println!("Hello: {}", v);
        None
    }
}

fn main() {
    let mut ps: Vec<Box<Process>> = Vec::new();
    ps.push(Box::new(Printer {}));
    for i in 0..25 {
        let x = Integral::new(i).handle().value();
        transducer::inges(&mut ps, &x);
    }

    let y = Integral::new(7).handle().value();
    let z = Integral::new(2).handle().value();
    //let w = y + z;
    //println!("Goodbye: {}", w);

    // TODO filter
    let mut v = new_vector();
    for i in 0..100i64 {
        //let w = new_vector().conj(Value::from(i + 1)).conj(Value::from(i + 2));
        //v = v.assoc(Value::from(i), Value::from(i + 1));
        v = v.conj(Value::from(i));
    }
    println!("Now v: {}", v);

    let cat = v.empty();
    println!("cat: {}", cat);

    println!("hash: 0x{:08X}", Value::from(9).hash());

    test_me();
}

