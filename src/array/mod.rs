// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Algorithms for arrays.
//!
//! Sorting arrays. Stable sort. Heap sort.
//! Algorithms on polynomials; convolution
//! Algorithms on matrices; sum, multiply
//! - linear algebra

// array:
//   bignum: arithmetic +-*/%(pow)mod bitwise ^|&<<>>
//   polynomial: evaluate, add, sub, convolve
//   matrix: add, sub, multiply, linear algebra-rref
// instances: byte, bool, int, float, long, double, object (value?)
//
pub struct BigIntegral {}

pub fn convolve(a: &[f64], b: &[f64]) -> Box<[f64]> {
    unimplemented!()
}

pub struct Matrix {}

pub fn multiply(a: Matrix, b: Matrix) -> Matrix {
    unimplemented!()
}

