// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::*;
use fress::memory::*;

fn main() {
    //let s = Segment::new(7);
    //dbg!(s[0]);
    //dbg!(s.unalias());
    //Segment::free(s);

    eprintln!("hash({:>2})", 8);
    let v: Value = 55.into();
    eprintln!("SOMETHING");
    let p = format!("v is {}", v);
    eprintln!("{}", p);

    use std::time::Instant;
    let now = Instant::now();
    {
    let mut w: Value = vector();
    for i in 0..100_000 {
        w = w.conj(i.into());
    }
    eprintln!("w.count() is {}", w.count());
    eprintln!("w is {}", w);
    }
    eprintln!("Took: {}", now.elapsed().as_millis());


    let now = Instant::now();
    {
    let mut w: Value = hash_map();
    for i in 0..20 {
        w = w.assoc(i.into(), (i*2).into());
    }
    eprintln!("w.count() is {}", w.count());
    eprintln!("w is {}", w);
    }
    eprintln!("Took: {}", now.elapsed().as_millis());
}
