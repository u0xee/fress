// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use memory;
use super::{Dispatch, Distributor};
use std::mem::transmute;

pub fn distributor<T: Dispatch>() -> Distributor {
    unsafe {
        let as_ref = &*(0 as *const T);
        let as_ob = as_ref as &Dispatch;
        let null_and_table = transmute::<&Dispatch, [Unit; 2]>(as_ob);
        assert_eq!(Unit::from(0), null_and_table[0]);
        Distributor { opaque_method_table_ptr: null_and_table[1] }
    }
}


pub fn as_dispatch_obj<'a>(distributor_base: *const Distributor) -> &'a Dispatch {
    let distributor = memory::get(Unit::from(distributor_base).into());
    let ptr_and_table: [Unit; 2] = [distributor_base.into(), distributor];
    unsafe {
        transmute::<[Unit; 2], &Dispatch>(ptr_and_table)
    }
}
