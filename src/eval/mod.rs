// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use value::Value;
use ::{read, read_or_err, vector};

pub mod structure;
pub mod compile;
pub mod assemble;
pub mod var;
pub mod func;
use handle::Handle;

#[link(wasm_import_module = "env")]
extern {
    // TODO more of a return, module - mem and table needs - notes
    //  notes including info to format the code sample
    fn wasm_compile_init(byte_address: u32, byte_count: u32, mem_base: u32, tab_base: u32);
    fn post_output(byte_address: u32, byte_count: u32);
    fn post_error(byte_address: u32, byte_count: u32);
}


pub struct Statics {
    pub sym_do: Value,
    pub sym_if: Value,
    pub sym_fn: Value,
    pub sym_def: Value,
    pub sym_let: Value,
    pub sym_loop: Value,
    pub sym_recur: Value,

    pub local_use: Value,
    pub forms_using: Value,
    pub refers_to: Value,
    pub resolved_from: Value,
    pub arity_bitmap: Value,
    pub arity_to_idx: Value,
    pub vararg: Value,
    pub captures: Value,

    pub sym_value: Value,
    pub sym_value_ref: Value,
    pub key_name: Value,
    pub key_args: Value,
    pub key_ret: Value,
    pub key_mapped: Value,
    pub key_alias: Value,
    pub sym_fress: Value,
    pub sym_amp: Value,
}
use std::cell::Cell;
thread_local! {
    pub static STATICS: Cell<usize> = Cell::new(0);
}
pub fn load_statics() {
    let x = Box::new(Statics {
        sym_do: read("do"),
        sym_if: read("if"),
        sym_fn: read("fn"),
        sym_def: read("def"),
        sym_let: read("let"),
        sym_loop: read("loop"),
        sym_recur: read("recur"),
        local_use: read(":local-use"),
        forms_using: read(":forms-using"),
        refers_to: read(":refers-to"),
        resolved_from: read(":resolved-from"),
        arity_bitmap: read(":arity-bitmap"),
        arity_to_idx: read(":arity-to-idx"),
        vararg: read(":vararg"),
        captures: read(":captures"),
        sym_value: read("value"),
        sym_value_ref: read("&value"),
        key_name: read(":name"),
        key_args: read(":args"),
        key_ret: read(":ret"),
        key_mapped: read(":mapped"),
        key_alias: read(":alias"),
        sym_fress: read("fress"),
        sym_amp: read("&"),
    });
    let y = Box::into_raw(x) as usize;
    STATICS.with(|c| c.set(y));
}
pub fn get_statics() -> &'static Statics {
    let u = STATICS.with(|c| c.get());
    unsafe { &*(u as *const Statics) }
}

#[no_mangle]
pub extern fn initialize_global_state() {
    use std::panic;
    panic::set_hook(Box::new(|msg| {
        let s = format!("{}", msg);
        ::trace::panic_error(&s);
    }));
    init();
}

pub fn init() {
    //group!("Init global state");
    load_statics();
    compile::init_primitives();
    var::init();
    //group_end!();
}

// TODO Entry points
//  Analysis (read analysis compile?) -> results, markup, colored, notes, errors
//   -> Read -> Error
//   -> Structure -> Error
//   -> Compile
//   -> Assemble
//   -> Load, Run -> Error
//   - Report: Markup on input, Run results. Compiled code stats
//  TAB Completion at cursor, "dictionary" of in-scope locals and globals (names, symbols)
//    - takes utf-8 buffer (before cursor), reads, structure
//  ENTER New line indent, read partial data structure, retrieve latest collection
//   Use position meta to compute indent.
//  EXPAND/MOVE Delimiting elements with byte-ranges, used for expand-selection
//    Can be just a list of byte-ranges, element nesting can be computed from the ranges.
//  CLOSE Close all open collections (ctrl-] or something)
//  Run nearest top-level form
#[no_mangle]
pub extern fn read_eval_print(byte_address: u32, byte_count: u32) {
    group!("Read-eval-print routine!");
    let m = _read_structure_compile_assemble(byte_address, byte_count);
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
        let res = read_or_err(s);
        match res {
            Ok(v) => v,
            Err(msg) => {
                unsafe {
                    post_error(msg.as_ptr() as u32, msg.len() as u32)
                }
                error!("{}", msg);
                unimplemented!();
            }
        }
    };
    group_end!();
    group!("Structuring");
    let structured = {
        let res = structure::structure(&v);
        match res {
            Ok(r) => r,
            Err(msg) => {
                unsafe {
                    post_error(msg.as_ptr() as u32, msg.len() as u32)
                }
                error!("{}", msg);
                unimplemented!();
            }
        }
    };
    /*
    unsafe {
        use meta;
        meta::do_print_meta();
        let print = structured.to_string();
        meta::end_print_meta();
        post_output(print.as_ptr() as u32, print.len() as u32)
    }
    */
    group_end!();
    group!("Compiling");
    let ctx = compile::compile_top_level(&structured);
    group_end!();
    compile::show_context(&ctx);
    group!("Assembling");
    let module = assemble::wasm_module(&ctx);
    group_end!();
    module
}

// The list:
// * fressian, duh
// * let loop fn def literals
// * great observability
// * coloring atomic types, uuid timestamp fields etc.
// * introductory material




