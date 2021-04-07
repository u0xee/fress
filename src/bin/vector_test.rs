// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;

use fress::Value;
use fress::memory::{schedule, segment};
use fress::vector::harness::{self, Op, Ops};

use std::panic;
use std::thread;

fn main() {
    let ops3 = Ops(vec![Op::New(289)],
                   vec![Op::Conj(7), Op::Conj(4), Op::Set{index: 5, elem: 11}],
                   vec![Op::Conj(9), Op::Set{index: 5, elem: 22}]);
    let ops = Ops(vec![Op::New(288)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    let ops1 = Ops(vec![Op::New(7)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    let ops2 = Ops(vec![Op::New(17)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    harness::explore_schedules(ops3, 3);
    /*
    let r = panic::catch_unwind(|| {
        println!("Ready");
        panic!("Go");
        42
    });
    println!("Steady");
    //panic::resume_unwind(r.unwrap_err());
    let x = r.unwrap_err().downcast::<&str>().unwrap();
    println!("{:?}", x);
    */
}
