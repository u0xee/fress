// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use value::Value;
use ::{read, is_aggregate, hash_map, hash_set, list, vector, nil, tru, fals};

pub mod structure;
pub mod compile;
pub mod assemble;
pub mod var;
use self::compile::Context;
use handle::Handle;

// eval global context map! Containing
// * repl context, current *ns*
// * active aliases
// * Base functions
//  * fress/conj [v v] -> v
//  * fress/retire [v] -> nil
//  * fress/split_out [i] -> v

// structure used for static init, global environment:
// map of all vars, (:kw?)
pub fn eval(v: Value) -> Value {
    let globals = read("{+ fress/+, conj fress/conj, *ns* user}").unwrap();
    let (structured, notes) = structure::structure(v, &globals).expect("Error during structure");
    let ctx = compile::compile_top_level(&structured, &notes);
    let module = assemble::wasm_module(&ctx);
    println!("Module: {} bytes {:02X?}", module.len(), &module);
    vector().conj(structured).conj(notes)
}

#[no_mangle]
pub extern fn empty_vector() -> u32 {
    let x = vector();
    Handle::from(x).unit().u32()
}

#[no_mangle]
pub extern fn conj(c: u32, v: u32) -> u32 {
    let coll = Unit::from(c).handle().value();
    let val = Unit::from(v).handle().value();
    let res = coll.conj(val);
    Handle::from(res).unit().u32()
}

#[no_mangle]
pub extern fn from_signed_i64(x: u64) -> u32 {
    let v = Value::from(x as i64);
    Handle::from(v).unit().u32()
}

