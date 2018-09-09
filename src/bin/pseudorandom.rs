// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;

use fress_rust::fuzz::{cycle, cycle_n};

fn main() {
    print_ten_from(1u64);
    println!();
    print_ten_from(!1u64);
    println!();
    print_ten_from(!cycle(1u64));
}

fn print_ten_from(mut x: u64) {
    for i in 0..10 {
        println!("{}th -> {:16X}", i, x);
        x = cycle(x);
    }
}
