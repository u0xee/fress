// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::Value;


fn main() {
    let mut v = fress::vector();
    for i in 0..100i64 {
        v = v.conj(Value::from(i));
    }
    println!("v: {}", v);
    v = v.with_meta(Value::from(true));
    println!("v.meta: {}", v.meta());
}

