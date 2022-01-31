// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use super::*;
use handle::Handle;
use std::mem::transmute;

pub fn prism<T: Dispatch>() -> Unit {
    unsafe {
        let as_ref = &*(0 as *const T);
        let as_ob = as_ref as &dyn Dispatch;
        let null_and_table = transmute::<&dyn Dispatch, [Unit; 2]>(as_ob);
        assert_eq!(Unit::from(0), null_and_table[0]);
        null_and_table[1]
    }
}

pub fn as_dispatch<'a>(prism: &'a Unit) -> &'a dyn Dispatch {
    let ptr_and_table: [Unit; 2] = [Unit::from(1), *prism]; // TODO
    unsafe {
        transmute::<[Unit; 2], &dyn Dispatch>(ptr_and_table)
    }
}

pub fn eq(prism: AnchoredLine, other: Unit) -> bool {
    let p = prism[0];
    as_dispatch(&p).eq(prism, other)
}

pub fn logical_value(prism: AnchoredLine) -> AnchoredLine {
    let p = prism[0];
    mechanism::as_dispatch(&p).logical_value(prism)
}

#[inline(never)]
pub fn tear_down(prism: AnchoredLine) {
    let p = prism[0];
    mechanism::as_dispatch(&p).tear_down(prism);
}

pub fn alias_components(prism: AnchoredLine) {
    let p = prism[0];
    mechanism::as_dispatch(&p).alias_components(prism);
}

pub fn hash(prism: AnchoredLine) -> u32 {
    let p = prism[0];
    mechanism::as_dispatch(&p).hash(prism)
}

