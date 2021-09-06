// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

#![feature(test)]
extern crate test;
use test::Bencher;

extern crate fress;
use fress::*;

#[bench]
fn vector_grow_to_30(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vector();
        for i in 0..30 {
            v = v.conj(i.into());
        }
    });
}

fn x() {
    let mut v = vec![];
    let mut ct = 10_000;
    for i in 0..ct {
        v.push(Value::from(i))
    }
}
#[bench]
fn vector_something(b: &mut Bencher) { b.iter(|| { x() }); }

