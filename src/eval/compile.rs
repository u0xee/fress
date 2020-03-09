// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, hash_map, hash_set, vector, nil};
use wasm;

#[derive(Debug)]
pub struct Func {
    pub argc: u32,
    pub localc: u32, // vec of local types [i32 i32 f64 etc]
    // captured {a 7, b 5}
    pub code_bytes: Vec<u8>,
    pub current_depth: u32,

    pub loop_point: u32,
    pub recur_targets: Vec<u32>, // [4 7]
    pub loop_terminals: Vec<u32>, // #{1 2 3}

    pub try_point: u32,
    pub try_pinned: Value, // #{1 2 3}
}

// Functionality on Context
// -constant data (constants)
// -register var->idx (globals)
// -register imported fns (external fns)
// -register vtable segment

// named   {c 2, d 5, e 9}  terminal #{c d}
// dormant #{1 2 3} shadowed by let/loop, or gathering up args to a call.
// vacant  #{4 7} or vector, previously terminal along this path

#[derive(Debug)]
pub struct Context {
    pub signatures: Value,
    pub import_v: Value,
    pub imports: Value, // {["a" "b"] 7}
    pub global_v: Value, // [_ _ my/var :kw "str"]
    pub globals: Value,
    pub constant_data: Vec<u8>,
    pub vtable: Value,
    pub funcs: Vec<Func>,
    pub curr_func: usize,
}

pub fn compile_top_level(form: &Value, notes: &Value) -> Box<Context> {
    let mut ctx = Box::new(Context {
        signatures: Default::default(),
        import_v: vector(),
        imports: hash_map(),
        global_v: vector().conj(nil()).conj(nil()),
        globals: hash_map(),
        constant_data: vec![],
        vtable: vector(),
        funcs: vec![Func { // static init function, takes global map (source of vars, :kw etc)
            argc: 1, localc: 0, code_bytes: vec![], current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: hash_set(),
        }, Func { // top level form
            argc: 0, localc: 0, code_bytes: vec![], current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: hash_set(),
        }],
        curr_func: 1,
    });
    let res = {
        let m = hash_map();
        let s = hash_set();
        let v = vector();
        comp(&mut ctx, form, notes, &m, &m, &s, &s, &v)
    };
    let value_sym = read("value ").unwrap();
    if res != value_sym {
        unimplemented!();
    }
    let init_f: &mut Func = ctx.funcs.get_mut(0).unwrap();
    init_f.code_bytes.push(wasm::Op::LOCAL);
    init_f.code_bytes.push(0u8);
    init_f.code_bytes.push(wasm::Op::END);
    let top_f: &mut Func = ctx.funcs.get_mut(1).unwrap();
    top_f.code_bytes.push(wasm::Op::END);
    ctx
}

pub fn register_import(ctx: &mut Context, description: &Value) -> u32 {
    let name_k = read(":name ").unwrap();
    let name = description.get(&name_k);
    let idx = ctx.imports.get(name).split_out();
    if idx.is_integral() {
        idx.as_i64() as u32
    } else {
        let ct = ctx.import_v.count();
        use std::mem::replace;
        let iv = replace(&mut ctx.import_v, nil());
        ctx.import_v = iv.conj(description.split_out());
        let i  = replace(&mut ctx.imports, nil());
        ctx.imports = i.assoc(name.split_out(), ct.into());
        ct
    }
}

pub fn comp(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
            named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    if form.is_symbol() {
        if form.has_namespace() {
            unimplemented!()
        } else {
            // local, capture
            unimplemented!()
        }
    }
    if form.is_list() {
        let first = form.peek();
        if first.is_symbol() {
            let s = format!("{}", first);
            if &s == "def" { return comp_def(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "fn" { return comp_fn(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "if" { return comp_if(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "do" { return comp_do(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "let" { return comp_let(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "loop" { return comp_loop(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "recur" { return comp_recur(ctx, form, notes, captured, named, terminal, dormant, vacant) }
            if &s == "quote" { return comp_recur(ctx, form, notes, captured, named, terminal, dormant, vacant) }
        }
        return comp_call(ctx, form, notes, captured, named, terminal, dormant, vacant)
    }
    if form.is_vector() { return comp_vector(ctx, form, notes, captured, named, terminal, dormant, vacant) }
    if form.is_set() { unimplemented!() }
    if form.is_map() { unimplemented!() }
    if form.is_integral() { return comp_integral(ctx, form, notes, captured, named, terminal, dormant, vacant) }
    // :kw 3.4 \c true nil
    unimplemented!("Compile expression {}", form)
}
pub fn comp_local(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                  named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // local x, if named and further if terminal, else captured
    // emit bytecode to access frame local or captured
    unimplemented!()
}
pub fn comp_var(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // context->register var->global idx, emit bytecode to load global (the var) and get it's value.
    unimplemented!()
}
pub fn comp_def(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // Compile expression, context->register var->global idx, emit bytecode in current func,
    // to load global (the var) and set it's value.
    unimplemented!()
}
pub fn comp_fn(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // multiple bodies, check distinct arity. Create captured set, assign idx. for each body
    // if recur, set up recur point like a loop
    // context->create func, use captured, named w/ just the args, no dormant no vacant.
    // use counts of args, for each form in body:
    // compute terminal args, compile exp (maybe clean up result), update counts by form usage.
    // emit bytecode to allocate closure, add captured values and indirect function indices.
    unimplemented!()
}
pub fn comp_if(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // compile test expression, computing terminals and vacant, emit bytecode for is_so and wasm-if
    // compute terminals for branches
    // if loop_terminals applies, add them to terminals or destroy them (if now shadowed)
    unimplemented!()
}
pub fn comp_do(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // for each terminal, use counts to compute terminal sub-expression, vacant afterwards.
    unimplemented!()
}
pub fn comp_let(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // for each terminal, use counts to compute terminal sub-expression.
    // for each binding, compile exp, update counts and recent vacant, allocate stack local.
    // emit bytecode to store to local. Use counts to determine when terminal.
    // if shadowing, save off shadowed to dormant set
    // for each form in body, cont. computing terminals and vacants.

    log!("notes {}", notes);
    // TODO
    unimplemented!()
}
pub fn comp_loop(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                 named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // like let, but establish loop-point and recur-targets
    // on recur, like call, build up stack of values, then set locals and branch back.
    unimplemented!()
}
pub fn comp_recur(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                  named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    unimplemented!()
}
pub fn comp_quote(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                  named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    unimplemented!()
}
pub fn comp_call(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                 named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // compile each expression in sequence, building up stack of values.
    // use counts to compute terminal and vacant.
    // on return, check for 0 to trigger cleanup and further unwinding
    // primitive calls
    // take mixtures of Value and &Value
    // nil?
    // conj
    let b = "{fress/conj {:name [\"fress\" \"conj\"]\
                                :args [value value]\
                                :ret  value}}";
    let base = read(b).unwrap();
    let first = form.peek();
    if first.is_symbol() && first.has_namespace() && base.contains(&first) {
        // (fress/conj [] 7)
        let base_fn = base.get(&first).split_out();
        let idx = register_import(ctx, &base_fn);
        let ct = form.count();
        for i in 1..ct {
            let e = form.nth(i);
            let r = comp(ctx, e, notes, captured, named, terminal, dormant, vacant);
        }
        let f: &mut Func = ctx.funcs.get_mut(ctx.curr_func).unwrap();
        f.code_bytes.push(wasm::Op::CALL);
        wasm::uleb128(&mut f.code_bytes, idx as u64);
        return read("value").unwrap();
    } else {
        // (x 2) (:kw m) (some/f 7) var reference
        unimplemented!()
    }
}
pub fn comp_vector(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                   named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // compile new-vector, then each expression, conjing in.
    let b = "{:name [\"fress\" \"new_vector\"]\
                    :args []\
                    :ret  value}";
    if form.is_empty() {
        let new_v = read(b).unwrap();
        let idx = register_import(ctx, &new_v);
        let f: &mut Func = ctx.funcs.get_mut(ctx.curr_func).unwrap();
        f.code_bytes.push(wasm::Op::CALL);
        wasm::uleb128(&mut f.code_bytes, idx as u64);
        return read("value").unwrap();
    }
    unimplemented!()
}
// constants :kw "str" \z true nil 4 1.5
// store bytes in const_data, either build at static-init, store in global, or build afresh.
pub fn comp_integral(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
                     named: &Value, terminal: &Value, dormant: &Value, vacant: &Value) -> Value {
    // if integral is small, call new_int(immediate) and return
    let b = "{:name [\"fress\" \"from_signed_i64\"]\
                    :args [i64]\
                    :ret  value}";
    let new_int = read(b).unwrap();
    let idx = register_import(ctx, &new_int);
    let f: &mut Func = ctx.funcs.get_mut(ctx.curr_func).unwrap();
    f.code_bytes.push(wasm::Op::I64_CONST);
    wasm::sleb128(&mut f.code_bytes, form.as_i64());
    f.code_bytes.push(wasm::Op::CALL);
    wasm::uleb128(&mut f.code_bytes, idx as u64);
    return read("value").unwrap();
}

