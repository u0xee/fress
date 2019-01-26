// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;
use fress_rust::value::{Value, ValueUnit};
use fress_rust::integral::Integral;
use fress_rust::map::Map;
use fress_rust::map::pop::Pop;
use fress_rust::memory::segment;

fn main() {
    /*
    let x = Integral::new(7);
    let y = Integral::new(10 - 3).value_unit();
    println!("Seven is {}. #{:08X}", x.value_unit(), x.value_unit().hash());
    println!("Y #{:08X}", y.hash());
    println!("x is == to y: {}", x.value_unit().eq(y));
    */

    let (new_a, free_a) = segment::new_free_counts();

    let limit = 1000;
    let mut m = Map::new().value_unit();
    for i in 0..limit {
        let k = Integral::new(i).value_unit();
        let v = Integral::new(i + 1).value_unit();
        m = m.assoc(k, v);
        //println!("#Associated {:2} to {:2}", i, i + 1);
    }
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
    for i in 0..limit {
        let k = Integral::new(i).value_unit();
        let v = m.get(k);
        print!("{} {}, ", k, v);
        k.retire();
    }
    println!();

    m.retire();
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}

