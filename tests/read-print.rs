// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::*;

// let repl: &'static str = env!("CARGO_BIN_EXE_repl");
// repl => .../fress/target/release/repl

#[test]
fn test_a() {
    assert_eq!(vector().conj(7.into()), vec![7].into());
}

