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
use std::io::Write;

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
    use std::fs::File;
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(&module).expect("Failed to write bytes to file.");
    vector().conj(structured).conj(notes)
}

#[link(wasm_import_module = "cool_js")]
extern {
    fn js_log_(byte_address: u32, byte_count: u32);
    fn js_error_(byte_address: u32, byte_count: u32);
    fn js_compile_init(byte_address: u32, byte_count: u32, mem_base: u32, tab_base: u32);
}
pub fn js_log(s: &str) {
    unsafe { js_log_(s.as_ptr() as usize as u32, s.len() as u32) }
}
pub fn js_error(s: &str) {
    unsafe { js_error_(s.as_ptr() as usize as u32, s.len() as u32); }
}

use std::cell::Cell;
thread_local! {
    pub static GLOBAL_RESOLVE: Cell<u32> = Cell::new(0);
}
#[no_mangle]
pub extern fn initialize_global_state() {
    use std::panic;
    panic::set_hook(Box::new(|msg| {
        let s = format!("{}", msg);
        js_error(&s);
    }));
    // Global state - resolution map, vars map, static memory pool, table pool
    let globals = read("{+ fress/+, conj fress/conj, *ns* user}").unwrap();
    let g = Handle::from(globals).unit.u32();
    GLOBAL_RESOLVE.with(|c| c.set(g));
}
#[no_mangle]
pub extern fn read_eval_print(byte_address: u32, byte_count: u32) {
    let v = {
        use std::slice;
        let bytes = unsafe {
            slice::from_raw_parts(byte_address as usize as *const u8,
                                  byte_count as usize)
        };
        use std::str;
        let s = str::from_utf8(bytes).unwrap();
        let res = read(s);
        match read(s) {
            Ok(v) => v,
            Err(msg) => {
                js_error(&msg);
                return;
            }
        }
    };
    let (structured, notes) = {
        let g: u32 = GLOBAL_RESOLVE.with(|c| c.get());
        let globals = Unit::from(g).handle().value();
        let res = structure::structure(v, &globals);
        Handle::from(globals);
        match res {
            Ok(r) => r,
            Err(msg) => {
                js_error(&msg);
                return;
            }
        }
    };
    let ctx = compile::compile_top_level(&structured, &notes);
    let module = assemble::wasm_module(&ctx);
    // TODO allocate static memory and table space
    unsafe {
        js_compile_init(module.as_ptr() as usize as u32, module.len() as u32,
                        0, 0)
    }
}

#[no_mangle]
pub extern fn new_vector() -> u32 {
    let x = vector();
    Handle::from(x).unit().u32()
}

#[no_mangle]
pub extern fn console_log(v: u32) {
    let val = Unit::from(v).handle().value();
    let s = format!("{}", val);
    js_log(&s);
    Handle::from(val);
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

