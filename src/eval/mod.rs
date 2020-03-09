// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use value::Value;
use ::{read, vector};

pub mod structure;
pub mod compile;
pub mod assemble;
pub mod var;
pub mod func;
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
    use std::fs::File;
    use std::io::Write;
    warn!("Writing to file..");
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(&module).expect("Failed to write bytes to file.");
    vector().conj(structured).conj(notes)
}

#[link(wasm_import_module = "env")]
extern {
    fn wasm_compile_init(byte_address: u32, byte_count: u32, mem_base: u32, tab_base: u32);
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
        ::trace::panic_error(&s);
    }));
    group!("Global state initialization");
    // Global state - resolution map, vars map, static memory pool, table pool
    let globals = read("{+ fress/+, conj fress/conj, *ns* user}").unwrap();
    let g = Handle::from(globals).unit.u32();
    GLOBAL_RESOLVE.with(|c| c.set(g));
    group_end!();
}

#[no_mangle]
pub extern fn read_eval_print(byte_address: u32, byte_count: u32) {
    group!("Read-eval-print routine");
    let m = _read_structure_compile_assemble(byte_address, byte_count);
    // TODO allocate static memory and table space
    unsafe {
        wasm_compile_init(m.as_ptr() as usize as u32, m.len() as u32,
                          0, 0)
    }
    group_end!();
}
pub fn _read_structure_compile_assemble(byte_address: u32, byte_count: u32) -> Vec<u8> {
    group!("Reading");
    let v = {
        use std::slice;
        let bytes = unsafe {
            slice::from_raw_parts(byte_address as usize as *const u8,
                                  byte_count as usize)
        };
        use std::str;
        let s = str::from_utf8(bytes).unwrap();
        let res = read(s);
        match res {
            Ok(v) => v,
            Err(msg) => {
                error!("{}", msg);
                unimplemented!();
            }
        }
    };
    group_end!();
    group!("Structuring");
    let (structured, notes) = {
        let g: u32 = GLOBAL_RESOLVE.with(|c| c.get());
        let globals = Unit::from(g).handle().value();
        let res = structure::structure(v, &globals);
        Handle::from(globals);
        match res {
            Ok(r) => r,
            Err(msg) => {
                error!("{}", msg);
                unimplemented!();
            }
        }
    };
    group_end!();
    group!("Compiling");
    let ctx = compile::compile_top_level(&structured, &notes);
    group_end!();
    group!("Assembling");
    let module = assemble::wasm_module(&ctx);
    group_end!();
    module
}

// What's next?
// group edn read events, eg reading-uuid, symbolic, number, vector etc
// structure, compile, assemble traces
// see the flow of compiler logic
// big arc, locals, fns, vars, recur, literals, :kw

// The list:
// fressian, duh
// let loop fn def literals
// great observability
// introductory material

// TODO
// Compiled code call tracing routines
// Locate trace messages with urls to rust doc
//

// TODO move these, library interface fns
#[no_mangle]
pub extern fn new_vector() -> u32 {
    let x = vector();
    Handle::from(x).unit().u32()
}

// TODO instead, value into js string
#[no_mangle]
pub extern fn console_log(v: u32) {
    let val = Unit::from(v).handle().value();
    log!("{}", val);
    Handle::from(val); // forget
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

