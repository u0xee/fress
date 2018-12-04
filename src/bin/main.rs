// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;
use fress_rust::value::{Value, ValueUnit};
use fress_rust::vector::Vector;

fn main() {
    println!("Hello, world!");
    let mut v = Vector::new().value_unit();
    println!("{:?}", v);
    let count = 10_000_000;
    for i in 0..count {
        v = v.conj(ValueUnit::num(i));
        //println!("{:?}", v);
    }
    println!("{:?}", v.nth(count - 10));
    /*
    for i in 0..count {
        v = v.pop();
    }
    println!("{:?}", v);
    */
    /*
    v.split();
    let w = v;
    println!("E{:?}", w);
    v = v.assoc(ValueUnit::num(3), ValueUnit::num(1));
    println!("{:?}", w);
    println!("{:?}", v);
    for i in 1..20 {
        println!("{}: {:?}", i, v.nth(i));
    }

    v.split();
    let w = v;
    println!("{:?}", w);
    v = v.conj(ValueUnit::num(7));
    println!("{:?}", v);
    println!("{:?}", w);

    for i in 1..10 {
        v = v.pop();
        println!("{:?}", v);
    }
    v.split();
    let mut y = v;
    println!("{:?}", v);
    y = y.conj(ValueUnit::num(3));
    println!("{:?}", y);
    y = y.conj(ValueUnit::num(4));
    y = y.conj(ValueUnit::num(5));
    println!("{:?}", y);
    println!("{:?}", v);

    let mut s = Segment::new(5);
    println!("{:?}", s);
    println!("{:?}", s[3]);
    s[3] = 7.into();
    println!("{:?}", s[3]);
    println!("{:?}", s);
    Segment::free(s);


    let v = Vector::new_value();
    println!("v = {:?}", Segment::from(v.handle));
    let v1 = v.conj(1.into());
    println!("v1 = {:?}", Segment::from(v1.handle));
    let v2 = v1.conj(2.into());
    let w = v2.conj(3.into());

    let (w_, three) = w.pop();
    let (w__, two) = w_.pop();
    let (w___, one) = w__.pop();
    assert_eq!(three, 3.into());
    assert_eq!(two, 2.into());
    assert_eq!(one, 1.into());

    println!("Goodbye, world!");
    */

}

