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

const CT: u32 = 100_000;

/*
#[bench]
fn vector_grow(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vector();
        for i in 0..CT {
            v = v.conj(Value::from(i));
        }
    });
}
*/
#[bench]
fn vec_grow(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vec![];
        for i in 0..CT {
            v.push(Value::from(i));
        }
    });
}
/*
#[bench]
fn create_destroy(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..CT {
            let x = Value::from(i);
        }
    });
}
#[bench]
fn vec_grow_with_cap(b: &mut Bencher) {
    b.iter(|| {
        let mut v = Vec::with_capacity(CT as usize);
        for i in 0..CT {
            v.push(Value::from(i));
        }
    });
}
*/

