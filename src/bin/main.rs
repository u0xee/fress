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
    xf.add_transducer(filter());
    let s = into(hash_set(), xf, v);
    println!("s: {}", s);
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

