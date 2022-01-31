// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, hash_map, vector};
use wasm;
use super::compile::{Func, Context};

pub fn wasm_module(ctx: &Context) -> Vec<u8> {
    let (sig_vec, imported_sigs, declared_sigs) = signatures(ctx);
    let signature_sec = signature_section(&sig_vec);
    let import_sec = import_section(ctx, imported_sigs);
    let declare_sec = declare_section(declared_sigs);
    let global_sec = global_section(ctx);
    let export_sec = export_section(ctx);
    let elem_sec = elem_section(ctx);
    let code_sec = code_section(ctx);

    let module = {
        let mut buf: Vec<u8> = vec![];
        buf.extend_from_slice(&wasm::MAGIC);
        buf.extend_from_slice(&wasm::VERSION);
        println!("Signature section: {:02X?}", &signature_sec);
        append_section(&mut buf, wasm::Section::FN_TYPE, &signature_sec);
        println!("Import section {}: {:02X?}", import_sec.len(), &import_sec);
        append_section(&mut buf, wasm::Section::IMPORT, &import_sec);
        println!("Declare section: {:02X?}", &declare_sec);
        append_section(&mut buf, wasm::Section::FN_DEC, &declare_sec);
        println!("Global section: {:02X?}", &global_sec);
        append_section(&mut buf, wasm::Section::GLOBAL, &global_sec);
        println!("Export section: {:02X?}", &export_sec);
        append_section(&mut buf, wasm::Section::EXPORT, &export_sec);
        println!("Elem section: {:02X?}", &elem_sec);
        append_section(&mut buf, wasm::Section::ELEM, &elem_sec);
        println!("Code section: {:02X?}", &code_sec);
        append_section(&mut buf, wasm::Section::CODE, &code_sec);
        append_data_section(&mut buf, &ctx.constant_data);
        buf
    };
    module
}

pub fn mem_and_table_needs(ctx: &Context) -> (u32, u32) {
    (ctx.constant_data.len() as u32, ctx.vtable.count())
}

pub fn append_section(buf: &mut Vec<u8>, sec_id: u8, sec: &[u8]) {
    buf.push(sec_id);
    wasm::uleb128(buf, sec.len() as u64);
    buf.extend_from_slice(sec);
}

pub fn signatures(ctx: &Context) -> (Value, Vec<u32>, Vec<u32>) {
    let args_key = read(":args ");
    let ret_key = read(":ret ");
    let type_map = read("{value i32, i32 i32, i64 i64}");
    let mut sigs = vector();
    let mut sig_map = hash_map();

    let imported_sig_ids = {
        let mut ids: Vec<u32> = vec![];
        let import_ct = ctx.import_v.count();
        for i in 0..import_ct {
            let desc = ctx.import_v.nth(i);
            let s = wasm_signature(&type_map, desc.get(&args_key), desc.get(&ret_key));
            let idx = sig_map.get(&s).split_out();
            let id = if idx.is_integral() {
                idx.as_i64() as u32
            } else {
                let sig_ct = sigs.count();
                sig_map = sig_map.assoc(s.split_out(), sig_ct.into());
                sigs = sigs.conj(s);
                sig_ct
            };
            ids.push(id);
        }
        ids
    };
    let declared_sig_ids = {
        let mut ids: Vec<u32> = vec![];
        let declare_ct = ctx.funcs.len();
        for i in 0..declare_ct {
            // TODO instead use local_slots and argc
            let argc = ctx.funcs.get(i).unwrap().argc;
            let s = argc_to_wasm_signature(argc);
            let idx = sig_map.get(&s).split_out();
            let id = if idx.is_integral() {
                idx.as_i64() as u32
            } else {
                let sig_ct = sigs.count();
                sig_map = sig_map.assoc(s.split_out(), sig_ct.into());
                sigs = sigs.conj(s);
                sig_ct
            };
            ids.push(id);
        }
        ids
    };
    (sigs, imported_sig_ids, declared_sig_ids)
}

pub fn wasm_signature(type_map: &Value, args: &Value, ret: &Value) -> Value {
    let mut arg_v = vector();
    let argc = args.count();
    for i in 0..argc {
        let num_type = type_map.get(args.nth(i)).split_out();
        arg_v = arg_v.conj(num_type);
    }
    let mut ret_v = vector();
    if !ret.is_nil() {
        ret_v = ret_v.conj(type_map.get(ret).split_out());
    }
    vector().conj(arg_v).conj(ret_v)
}

pub fn argc_to_wasm_signature(argc: u32) -> Value {
    let sym_i32 = read("i32 ");
    let mut arg_v = vector();
    for i in 0..argc {
        arg_v = arg_v.conj(sym_i32.split_out());
    }
    let ret = vector().conj(arg_v).conj(vector().conj(sym_i32));
    log!("argc {}, wasm signature {}", argc, &ret);
    ret
}

pub fn signature_section(sigs: &Value) -> Vec<u8> {
    let type_map = read("{i32 0x7F, i64 0x7E, f32 0x7D, f64 0x7C}");
    let mut buf: Vec<u8> = vec![];
    let sig_ct = sigs.count();
    wasm::uleb128(&mut buf, sig_ct as u64);
    for i in 0..sig_ct {
        let s = sigs.nth(i);
        println!("{}: {}", i, s);
        signature_section_each(&mut buf, &type_map, s.nth(0), s.nth(1));
    }
    buf
}

pub fn signature_section_each(buf: &mut Vec<u8>, type_map: &Value, args: &Value, ret: &Value) {
    buf.push(wasm::Type::FN);
    let argc = args.count();
    buf.push(argc as u8);
    for i in 0..argc {
        let num_type = type_map.get(args.nth(i)).as_i64() as u8;
        buf.push(num_type);
    }
    if ret.is_empty() {
        buf.push(0u8);
    } else {
        buf.push(1u8);
        let num_type = type_map.get(ret.nth(0)).as_i64() as u8;
        buf.push(num_type);
    }
}

pub fn import_section(ctx: &Context, sigs: Vec<u32>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let total_ct = 4 /*memory, table, bases*/ + ctx.import_v.count();
    wasm::uleb128(&mut buf, total_ct as u64);
    {
        let mem = read("[\"sys\" \"memory\"]");
        name_pair_to_buf(&mut buf, &mem);
        buf.push(2u8); // memory
        buf.push(wasm::Type::MIN);
        buf.push(0u8);

        let mem_base = read("[\"sys\" \"memory_base\"]");
        name_pair_to_buf(&mut buf, &mem_base);
        buf.push(3u8); // global
        buf.push(wasm::Type::I32);
        buf.push(wasm::Type::GLOBAL_CONST);
    }
    {
        let tab = read("[\"sys\" \"table\"]");
        name_pair_to_buf(&mut buf, &tab);
        buf.push(1u8); // table
        buf.push(wasm::Type::FN_REF);
        buf.push(wasm::Type::MIN);
        buf.push(0u8);

        let tab_base = read("[\"sys\" \"table_base\"]");
        name_pair_to_buf(&mut buf, &tab_base);
        buf.push(3u8); // global
        buf.push(wasm::Type::I32);
        buf.push(wasm::Type::GLOBAL_CONST);
    }
    {
        let name_key = read(":name ");
        let import_ct = ctx.import_v.count();
        for i in 0..import_ct {
            let name = ctx.import_v.nth(i).get(&name_key);
            name_pair_to_buf(&mut buf, name);
            buf.push(0u8); // function
            let sig_id = sigs[i as usize];
            wasm::uleb128(&mut buf, sig_id as u64);
        }
    }
    buf
}

pub fn name_pair_to_buf(buf: &mut Vec<u8>, name_pair: &Value) {
    name_to_buf(buf, name_pair.nth(0));
    name_to_buf(buf, name_pair.nth(1));
}

pub fn name_to_buf(buf: &mut Vec<u8>, name: &Value) {
    use string;
    let prism = string::find_prism(name._handle()).unwrap();
    let bytes = string::byte_slice(&prism);
    wasm::uleb128(buf, bytes.len() as u64);
    buf.extend_from_slice(bytes);
}

pub fn declare_section(sigs: Vec<u32>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    wasm::uleb128(&mut buf, sigs.len() as u64);
    for idx in sigs {
        wasm::uleb128(&mut buf, idx as u64);
    }
    buf
}

pub fn global_section(ctx: &Context) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let ct = ctx.globals.count(); // not counting imported globals
    wasm::uleb128(&mut buf, ct as u64);
    for i in 0..ct {
        buf.push(wasm::Type::I32);
        buf.push(wasm::Type::GLOBAL_MUT);
        buf.push(wasm::Op::I32_CONST);
        buf.push(0u8); // placeholder, will be initialized on static_init
        buf.push(wasm::Op::END);
    }
    buf
}

pub fn export_section(ctx: &Context) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let import_ct = ctx.import_v.count();
    let export_ct = 2u8;
    buf.push(export_ct);
    let static_init = b"static_init";
    buf.push(static_init.len() as u8);
    buf.extend_from_slice(static_init);
    buf.push(0u8);
    wasm::uleb128(&mut buf, import_ct as u64);
    let main = b"main";
    buf.push(main.len() as u8);
    buf.extend_from_slice(main);
    buf.push(0u8);
    wasm::uleb128(&mut buf, import_ct as u64 + 1);
    buf
}

pub fn elem_section(ctx: &Context) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    buf.push(1u8); // segment count
    buf.push(0u8); // table 0
    buf.push(wasm::Op::GLOBAL);
    buf.push(1u8); // global 1, table base
    buf.push(wasm::Op::END);
    buf.push(0u8); // TODO empty vector for now
    // let import_ct = ctx.import_v.count();
    // for each vtable local entry, add to import_ct and write idx
    buf
}

pub fn code_section(ctx: &Context) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let func_ct = ctx.funcs.len() as u32;
    wasm::uleb128(&mut buf, func_ct as u64);
    for i in 0..func_ct {
        let f = ctx.funcs.get(i as usize).unwrap();
        let local_decl = local_declaration(f);
        let size = local_decl.len() as u32 + f.code_bytes.len() as u32;
        wasm::uleb128(&mut buf, size as u64);
        buf.extend_from_slice(&local_decl);
        buf.extend_from_slice(&f.code_bytes);
    }
    buf
}

pub fn local_declaration(f: &Func) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    // TODO if no locals, run_ct should be 0
    // use f.local_slots && f.argc to get locals (not args) array
    // chop into runs
    let run_ct = 1u8;
    buf.push(run_ct);
    let local_ct = 0;
    buf.push(local_ct as u8);
    buf.push(wasm::Type::I32); // single run of i32s
    buf
}

pub fn append_data_section(buf: &mut Vec<u8>, constant_data: &[u8]) {
    let bytes_ct = constant_data.len() as u32;
    let sec_header = {
        let mut b: Vec<u8> = Vec::with_capacity(10);
        b.push(1u8); // segment count
        b.push(0u8); // memory 0
        b.push(wasm::Op::GLOBAL);
        b.push(0u8); // global 0, memory base
        b.push(wasm::Op::END);
        wasm::uleb128(&mut b, bytes_ct as u64);
        b
    };
    let total_size = sec_header.len() as u32 + bytes_ct;
    buf.push(wasm::Section::DATA);
    wasm::uleb128(buf, total_size as u64);
    buf.extend_from_slice(&sec_header);
    buf.extend_from_slice(constant_data);
}

