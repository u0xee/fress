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
    use fress_rust::memory::segment;
    use fress_rust::integral::Integral;

    let (new_a, free_a) = segment::new_free_counts();

    let count = 20000;
    let mut v = Vector::new().value_unit();
    /*
    let mut z = Vector::new().value_unit();
    for i in 0..10 {
        z = z.conj(Integral::new(i).value_unit());
    }
    z.tear_down();
    */
    for i in 0..count {
        v = v.conj(Integral::new(i).value_unit());
    }
    v.split();
    let mut w = v;
    w = w.conj(Integral::new(5).value_unit());
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
    for i in 0..(count - 5) {
        v = v.assoc(ValueUnit::num(i as u32), Integral::new(20).value_unit());
    }
    {
        v.retire();
        w.retire();
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
    /*for i in 0..count {
        print!("{} ", v.nth(i));
    }*/

    //v.segment().interactive_print_bits("V before");
    /*
    v.split();
    let mut w = v;
    w = w.conj(Integral::new(5).value_unit());
    v = v.conj(Integral::new(6).value_unit());

    //v.segment().interactive_print_bits("V after split and conj");
    //w.segment().interactive_print_bits("W after split and conj");

    for i in 0..(count - 5) {
        v = v.assoc(ValueUnit::num(i as u32), Integral::new(i & !0x1).value_unit());
    }
    */
    /*for i in 0..(count - 5) {
        print!("v{:?} ", v.nth(i));
    }*/
    /*for i in 0..(count - 5) {
        print!("w{:?} ", w.nth(i));
    }*/
    //v = v.pop();



    /*for i in 0..(count - 10000) {
        w = w.pop();
    }

    for i in 0..(count - 70000) {
        v = v.pop();
    }*/

    //w.tear_down();
    //v.tear_down();
    /*
    let (new_b, free_b) = segment::new_free_counts();
    let new_diff = new_b - new_a;
    let free_diff = free_b - free_a;
    println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    */

    /*
    for round in 0..5 {
        for i in 0..count {
            v = v.conj(ValueUnit::num(i));
        }
        v.split();
        let mut z = v;
        for i in 0..50 {
            z = z.conj(ValueUnit::num(5));
        }
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
    */
}

