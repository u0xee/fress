// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;

use fress_rust::fuzz::{cycle, cycle_n, exponent_offset, uniform_f64, normal_f64};

fn main() {
    /*
    print_ten_from(1u64);
    println!();
    print_ten_from(!1u64);
    println!();
    print_ten_from(!cycle(1u64));

    print_float(0.0f64);
    print_float(-0.0f64);
    print_float(0.5f64);
    print_float(1.0f64);
    print_float(2.0f64);
    print_float(std::f64::INFINITY);
    print_float(std::f64::NEG_INFINITY);
    print_float(std::f64::NAN);
    */

    //fstats(cycle_n(1, 10));
    normal_stats(cycle_n(1, 100));
}

fn normal_stats(mut seed: u64) {
    let mut counts = [0; 3];
    let samples = 10000000;
    for _ in 0..samples {
        let f = normal_f64(seed);
        let abs_f = f.abs();
        if abs_f < 1.0 {
            counts[0] += 1;
        } else if abs_f < 2.0 {
            counts[1] += 1;
        } else if abs_f < 3.0 {
            counts[2] += 1;
        }
        seed = cycle(seed);
    }
    for i in 0..3 {
        let x = counts[i as usize];
        println!("[{}] => {:.6} percent", i, (x as f64 * 100f64) / (samples as f64));
    }
}

fn fstats(mut seed: u64) {
    let mut counts = [0; 100];
    let samples = 1000000;
    for i in 0..samples {
        let f = uniform_f64(seed, cycle(seed));
        let x = f * 100f64; // percentile
        if x < 1.0f64 {
            let xx = x * 100f64; // hundredths of percent
            counts[xx as usize] += 1;
        }
        seed = cycle_n(seed,2);
    }
    for i in 0..100 {
        let x = counts[i as usize];
        println!("[{}] => {} ({:.6} percent)", i, x, (x as f64 * 10000f64) / (samples as f64));
    }
}

fn stats(mut seed: u64) {
    let mut counts = [0; 30];
    let samples = 1000000;
    for i in 0..samples {
        let x = exponent_offset(seed);
        counts[x as usize] += 1;
        seed = cycle(seed);
    }
    for i in 0..30 {
        let x = counts[i as usize];
        println!("[{}] => {} ({:.6} percent)", i, x, (x as f64 * 100f64) / (samples as f64));
    }
}

fn print_float(x: f64) {
    println!("Float print: {:16X}", x.to_bits());
}

fn print_ten_from(mut x: u64) {
    for i in 0..10 {
        println!("{}th -> {:16X}", i, x);
        x = cycle(x);
    }
}
