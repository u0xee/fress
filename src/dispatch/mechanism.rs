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
