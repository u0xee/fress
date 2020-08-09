// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::Unit;
use value::Value;
use ::{read, hash_map, hash_set, vector, nil};
use ::{wasm, right_into};

use super::get_statics;

#[derive(Debug)]
pub struct Func {
    pub argc: u32,
    pub local_slots: Vec<u8>, // wasm::Type::I32 etc
    pub captured: Value, // {a 7, b 5, fn_name -1}

    pub current_depth: u32,
    pub code_bytes: Vec<u8>,
    pub code_log: Vec<String>,

    pub loop_point: u32,
    pub recur_targets: Vec<u32>, // [4 7]
    pub loop_terminals: Vec<u32>, // #{1 2 3}

    pub try_point: u32,
    pub try_pinned: Value, // #{1 2 3}
}

#[derive(Debug)]
pub struct Context {
    pub signatures: Value,

    pub import_v: Value,
    pub imports: Value, // {["fress" "nil"] 7}

    pub global_v: Value, // [_ _ my/var :as "Hello, World!"]
    pub globals: Value,

    pub constant_data: Vec<u8>,
    pub vtable: Value,

    pub funcs: Vec<Func>,
    pub curr_func: usize,
}

pub fn show_context(ctx: &Context) {
    // show imported functions
    // show globals
    // show constant bytes, and vtable
    // show functions:
    //  static_init function
    //  main function
    //  each additional function (args, locals, captured, bytecode bodies)
}

pub fn compile_top_level(form: &Value) -> Box<Context> {
    let mut ctx = Box::new(Context {
        signatures: Default::default(),
        import_v: vector(),
        imports: hash_map(),
        global_v: vector().conj(nil()).conj(nil()),
        globals: hash_map(),
        constant_data: vec![],
        vtable: vector(),
        funcs: vec![Func {
            argc: 0, captured: nil(), local_slots: vec![], code_bytes: vec![], code_log: vec![],
            current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: nil(),
        }, Func { // top level form
            argc: 0, captured: nil(), local_slots: vec![], code_bytes: vec![], code_log: vec![],
            current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: nil(),
        }],
        curr_func: 1,
    });
    let res = {
        let m = hash_map();
        let s = hash_set();
        let v = vector();
        comp(&mut ctx, form, &m, &s, &s, &v, 0)
    };
    let value_sym = read("value ").unwrap();
    if res != value_sym {
        unimplemented!();
    }
    let init_f: &mut Func = ctx.funcs.get_mut(0).unwrap();
    init_f.code_bytes.push(wasm::Op::I32_CONST);
    init_f.code_bytes.push(0u8);
    init_f.code_bytes.push(wasm::Op::END); // returning 0
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

// Context Current-Fn captures, wasm locals model. recur and try support: depth, local participation.
// named {terminal} map stack locals. dormant supports precise cleanup on unwind.
// vacant, next-local, and wasm locals model support allocating, and repurposing wasm stack locals
// next-local and vacant are path dependent. but wasm local definitions are function global
// No reuse mode, wasm fn level register fill. First come first served across all paths.
// With reuse, pull from vacant pool if possible, else use next_local looking for
// a wasm local slot that is the right type.

// Primitive function:
//  canonical name: fress/get
//  arguments: [&value &value]
//  return: value
// Ref coming in lifetime of: static capture local

// TODO Compile recursion protocol
//  Compiling context, form
//   Current function, captured, locals model, loop and try context
//   named   {c 2, d 5, e 9}  terminal #{c d}
//   dormant #{1 2 3} shadowed by let/loop, or gathering up args to a call or recur
//   vacant  #[4 7] previously terminal along this path
//   next_local 5
//  Return what's on the stack, value or &value -> byte code for type
pub fn comp(ctx: &mut Context, form: &Value, named: &Value, terminal: &Value,
            dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    if form.is_symbol() {
        if form.has_namespace() {
            // context->register var->global idx, emit bytecode to load global (the var) and get it's value.
            unimplemented!()
        } else {
            if named.contains(form) {
                // wasm stack local, emit bytecode to load local onto stack
                if terminal.contains(form) {
                    // value, terminal usage of a local, local is now unused. Bytecode to clear local?
                    let f: &mut Func = ctx.funcs.get_mut(ctx.curr_func).unwrap();
                    f.code_bytes.push(wasm::Op::LOCAL);
                    wasm::uleb128(&mut f.code_bytes, named.get(form).as_i64() as u64);
                } else {
                    // &value based on non-terminal local
                }
            } else { // capture
                let f: &mut Func = ctx.funcs.get_mut(ctx.curr_func).unwrap();
                let capture_idx = f.captured.get(form).as_i64();
                // bytecode or function call to access capture
            }
            unimplemented!()
        }
    }
    if form.is_list() {
        let first = form.peek();
        if first.is_symbol() {
            let s = format!("{}", first);
            if &s == "def" { return comp_def(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "fn" { return comp_fn(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "if" { return comp_if(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "do" { return comp_do(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "let" { return comp_let(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "loop" { return comp_loop(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "recur" { return comp_recur(ctx, form, named, terminal, dormant, vacant, next_local) }
            if &s == "quote" { return comp_recur(ctx, form, named, terminal, dormant, vacant, next_local) }
        }
        return comp_call(ctx, form, named, terminal, dormant, vacant, next_local)
    }
    if form.is_vector() { return comp_vector(ctx, form, named, terminal, dormant, vacant, next_local) }
    if form.is_set() { unimplemented!() }
    if form.is_map() { unimplemented!() }
    if form.is_integral() { return comp_integral(ctx, form, named, terminal, dormant, vacant, next_local) }
    // :kw 3.4 \c true nil
    unimplemented!("Compile expression {}", form)
}
pub fn comp_def(ctx: &mut Context, form: &Value,
                named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // Compile expression, context->register var->global idx, emit bytecode in current func,
    // to load global (the var) and set it's value.
    let x = comp(ctx, form.nth(2), named, terminal, dormant, vacant, next_local);
    unimplemented!()
}
pub fn comp_fn(ctx: &mut Context, form: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // (def square (fn [x] (* x x)))
    /*
    let captured = notes.get(form);
    let captured_map = xyz(captured);
    let body_ct = z();
    let first_func = ctx.funcs.len();
    for i in 0..body_ct {
        ctx.funcs.push(Func {
            argc: 0, captured: captured_map, local_slots: vec![], code_bytes: vec![], code_log: vec![],
            current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: nil(),
        });
        ctx.vtable = ctx.vtable.conj(Value::from(first_func + i));
    }
    let orig_curr_func = ctx.curr_func;
    for i in 0..body_ct {
        let body = form;
        let args = body.nth(0);
        // if recur, set up recur point like a loop
        ctx.curr_func = first_func + i;
        let b = ctx.funcs.get_mut(ctx.curr_func).unwrap();
        b.argc = 2 + args.count();
        for j in 0..b.argc {
            b.local_slots.push(wasm::Type::I32);
        }
        let named_args = named_from_args(args);
        let args_use_ct = notes.get(body);
        let expression_ct = body.count() - 1;
        for k in 0..expression_ct {
            // compute terminals
            let res = comp();
            // update counts, vacant
        }
    }
    ctx.curr_func = orig_curr_func;
    */
    // emit bytecode to allocate closure
    // install meta info, indirect fn indices
    // each captured value, access in current env and store in closure
    unimplemented!()
}
pub fn comp_if(ctx: &mut Context, form: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // compile test expression, computing terminals and vacant, emit bytecode for is_so and wasm-if
    let locals_used = form.meta().get(&get_statics().local_use).split_out();
    // make a map of each terminal to its local use count
    // wasm::Op::IF, wasm::Op::ELSE, wasm::Op::END
    //let test = comp(ctx, form.nth(1), named, xx, dormant, vacant, next_local);
    // compute terminals for branches
    // if loop_terminals applies, add them to terminals or destroy them (if now shadowed)
    unimplemented!()
}

pub fn count_down(terminals_used: &mut Value, subform: &Value) -> Value {
    unimplemented!()
    // subform_terminals_set
}
pub fn comp_body(ctx: &mut Context, form: &Value, named: &Value, terminals_used: &Value,
                 dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    let mut terminals_used = terminals_used.split_out(); // {a 3, b 1, d 1}
    let mut vac = vacant.split_out();
    let mut nam = named.split_out();
    let ct = form.count();
    for i in 0..ct {
        let subform_terms = count_down(&mut terminals_used, form.nth(i));
        let on_stack = comp(ctx, form.nth(i), &nam,
                            &subform_terms, dormant, &vac, next_local);
        // add subform_terminals' local slots to vacant
        // remove subform_terminals from named
        // if not the last subform, tear down value on_stack
    }
    unimplemented!()
}
pub fn comp_do(ctx: &mut Context, form: &Value,
               named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {

    unimplemented!()
}
pub fn comp_let(ctx: &mut Context, form: &Value,
                named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // for each terminal, use counts to compute terminal sub-expression
    // (let [a 5] (conj [] a))
    // for each binding, compile exp, update counts and recent vacant, allocate stack local.
    // emit bytecode to store to local. Use counts to determine when terminal.
    // if shadowing, save off shadowed to dormant set
    // for each form in body, cont. computing terminals and vacants.

    unimplemented!()
}
pub fn comp_loop(ctx: &mut Context, form: &Value,
                 named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // like let, but establish loop-point and recur-targets
    // on recur, like call, build up stack of values, then set locals and branch back.
    unimplemented!()
}
pub fn comp_recur(ctx: &mut Context, form: &Value,
                  named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    unimplemented!()
}
pub fn comp_quote(ctx: &mut Context, form: &Value,
                  named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    unimplemented!()
}
pub fn comp_call(ctx: &mut Context, form: &Value,
                 named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
    // compile each expression in sequence, building up stack of values.
    // use counts to compute terminal and vacant.
    // on return, check for 0 to trigger cleanup and further unwinding
    // primitive calls
    // take mixtures of Value and &Value
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
            // TODO next_local
            let r = comp(ctx, e, named, terminal, dormant, vacant, 0);
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
pub fn comp_vector(ctx: &mut Context, form: &Value,
                   named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
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
pub fn comp_integral(ctx: &mut Context, form: &Value,
                     named: &Value, terminal: &Value, dormant: &Value, vacant: &Value, next_local: u32) -> Value {
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


use std::cell::Cell;
thread_local! {
    pub static PRIMITIVES: Cell<Unit> = Cell::new(Unit { word: 0});
}
pub fn init_primitives() {
    let x = read(PRIMITIVE_INIT).unwrap()._consume().unit();
    PRIMITIVES.with(|c| c.set(x));
}
pub fn get_primitives() -> &'static Value {
    let x = PRIMITIVES.with(|c| c.as_ptr());
    unsafe { &*(x as usize as *const Value) }
}

static PRIMITIVE_INIT: &'static str = "\
{fress/conj {:name [\"fress\" \"conj\"]\
             :args [value value]\
             :ret  value},
 fress/nil? {:name [\"fress\" \"nil?\"]\
             :args [&value]\
             :ret  value}}
";

#[no_mangle]
pub extern fn new_vector() -> u32 {
    let x = vector();
    x._consume().unit().u32()
}

// TODO instead, value into js string
#[no_mangle]
pub extern fn console_log(v: u32) {
    let val = Unit::from(v).handle();
    log!("{}", val);
}

#[no_mangle]
pub extern fn conj(c: u32, v: u32) -> u32 {
    let coll = Unit::from(c).handle().value();
    let val = Unit::from(v).handle().value();
    let res = coll.conj(val);
    res._consume().unit().u32()
}

#[no_mangle]
pub extern fn from_signed_i64(x: u64) -> u32 {
    let v = Value::from(x as i64);
    v._consume().unit().u32()
}

