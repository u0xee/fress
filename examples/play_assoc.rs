// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::value::Value;
use fress::handle::Handle;
use fress::memory::segment;
use fress::integral;
use fress::set;
use fress::map;

fn main() {
    let (new_a, free_a) = segment::new_free_counts();

    let limit = 100_000;
    let mut s = set::new().handle();

    for i in 0..limit {
        if i % 4 == 0 {
            continue
        }
        let k = integral::new(i).handle();
        s = s.conj(k);
    }

    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }

    /*
    let ma = m;
    m.split();

    for i in 0..(limit >> 1) {
        let k = Integral::new(i).handle();
        let v = Integral::new(i + 2).handle();
        m = m.assoc(k, v);
    }

    for i in (limit >> 2)..(limit - (limit >> 2)) {
        let k = Integral::new(i).handle();
        let v = m.get(k);
        print!("{} {}, ", k, v);
        k.retire();
    }

    ma.retire();
    */
    let sa = s;
    s.split();

    for i in 50..90 {
        let k = integral::new(i).handle();
        s = s.dissoc(k);
        k.retire();
    }

    for i in 0..100 {
        let k = integral::new(i).handle();
        println!("Contains {}: {}", i, s.contains(k));
        k.retire();
    }

    s.retire();
    sa.retire();

    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("\nNew diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}

