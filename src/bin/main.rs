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
    let count = 2000;
    let mut v = Vector::new().value_unit();
    for i in 0..count {
        v = v.conj(ValueUnit::num(i));
    }
    for i in 0..count {
        print!("{} ", v.nth(i));
    }

    v.segment().interactive_print_bits("V before");

    v.split();
    let mut w = v;
    w = w.conj(ValueUnit::num(5));
    v = v.conj(ValueUnit::num(6));

    v.segment().interactive_print_bits("V after split and conj");
    w.segment().interactive_print_bits("W after split and conj");

    for i in 0..(count - 5) {
        v = v.assoc(ValueUnit::num(i), ValueUnit::num(i & !0x1))
    }
    for i in 0..(count - 5) {
        print!("v{:?} ", v.nth(i));
    }
    for i in 0..(count - 5) {
        print!("w{:?} ", w.nth(i));
    }

    println!("\nXXXXXXXXX0x{:X}", count - 1);
    v = v.pop();
    v.segment().print_bits();
    let tail_seg = v.segment().get(2).segment();
    tail_seg.print_bits();
    use fress_rust::memory::segment;
    //segment::please_save(tail_seg);
    w.segment().print_bits();
    for i in 0..(count - 3) {
        w = w.pop();
    }
    use fress_rust::fuzz;
    let freed = fuzz::log_copy();
    let tail_name = format!("{:X}", v.segment().get(2).u());
    for (i, s) in freed.iter().enumerate() {
        if s.contains(&tail_name) {
            println!("Did free {} as number {} in {} total",
                     tail_name, i, freed.len());
        }
    }
    println!("\nYYYYYYYYYYYY");
    for i in 0..(count - 3) {
        v = v.pop();
    }
    println!("{:?}", v);
    for round in 0..100 {
        for i in 0..count {
            v = v.conj(ValueUnit::num(i));
        }
        v.split();
        let mut z = v;
        z = z.conj(ValueUnit::num(5));
        v = v.conj(ValueUnit::num(5));
        for i in 0..(count/4) {
            v = v.pop();
        }
        for i in 0..(count/100) {
            z = z.assoc(ValueUnit::num(i * 100), ValueUnit::num(i))
        }
        while z.count() > 0 {
            z = z.pop();
        }
        while v.count() > 0 {
            v = v.pop();
        }
        println!("{:?}", v);
    }
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

