// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;
use fress_rust::value::Value;
use fress_rust::handle::Handle;
use fress_rust::integral::Integral;
use fress_rust::transducer;
use fress_rust::transducer::{Process};
use fress_rust::value::{new_vector, new_list, new_map, new_set};

pub struct Printer {}
impl Process for Printer {
    fn ingest(&mut self, process_stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
        println!("Hello: {}", v);
        None
    }
}

fn main() {
    let mut ps: Vec<Box<Process>> = Vec::new();
    ps.push(Box::new(Printer {}));
    for i in 0..25 {
        let x = Integral::new(i).handle().value();
        transducer::ingest(&mut ps, &x);
    }

    let y = Integral::new(7).handle().value();
    let z = Integral::new(2).handle().value();
    //let w = y + z;
    //println!("Goodbye: {}", w);


    let mut v = new_map();
    for i in 0..100i64 {
        v = v.assoc(Value::from(i), Value::from(i + 1));
    }
    println!("Now v: {}", v);

    let cat = v.empty();
    println!("cat: {}", cat);
}

