// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::*;
use fress::value::Value;
use fress::handle::Handle;
use fress::vector::Vector;
use fress::memory::segment;
use fress::transduce::Transducers;


fn m() {
    let v = vector().conj(Value::from(7)).conj(Value::from(7)).conj(Value::from(8))
        .conj(nil()).conj(tru()).conj(fals());
    println!("v: {}", v);
    let mut xf = Transducers::new();
    xf.add_transducer(filter(|v| {
        //let not_this = Value::from(8);
        //v.eq(&not_this)
        v.is_so()
    }));
    //let s = into(hash_set(), filter(|v| v.is_so()), v);
    let s = into(hash_set(), xf, v);
    println!("s: {}", s);

    for i in -5..5 {
        println!("{}", i % 3);
    }

    use fress::string::Str;
    let h = Str::new_value_from_str("");
    let t = Str::new_value_from_str("H");
    println!("h == t: {}", h == t);
    println!("h > t: {}", h > t);
    println!("Here's a string: {}", h);
    println!("Here's a hash: 0x{:08X}", h.hash());

    let m = {
        let k = Str::new_value_from_str("cats");
        let val = Value::from(7);
        let k2 = Str::new_value_from_str("dogs");
        let val2 = Value::from(5);
        hash_map().assoc(k, val).assoc(k2, val2)
    };
    println!("m: {}", m);
}

fn main() {
    let (new_a, free_a) = segment::new_free_counts();
    m();
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}

