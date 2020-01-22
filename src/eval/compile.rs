// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, is_aggregate, hash_map, hash_set, list, vector, nil, tru, fals};
use right_into;
use transduce::Process;


pub struct Func {
    pub argc: u32,
    pub localc: u32, // vec of local types [i32 i32 f64 etc]
    pub code_bytes: Vec<u8>,
    pub current_depth: u32,

    pub loop_point: u32,
    pub recur_targets: Vec<u32>, // [4 7]
    pub loop_terminals: Vec<u32>, // #{1 2 3}

    pub try_point: u32,
    pub try_pinned: Value, // #{1 2 3}
}

pub struct Context {
    pub signatures: Value,
    pub imports: Value, // {a/b 7}
    pub globals: Value,
    pub vars: Value,
    pub constant_data: Vec<u8>,
    pub vtable: Value,
    pub funcs: Vec<Func>,
    pub curr_func: u32,
}

pub fn compile_top_level(form: &Value, notes: &Value) -> Box<Context> {
    let mut ctx = Box::new(Context {
        signatures: Default::default(),
        imports: Default::default(),
        globals: Default::default(),
        vars: Default::default(),
        constant_data: vec![],
        vtable: Default::default(),
        funcs: vec![Func { // static init function, takes global map (source of vars etc)
            argc: 1, localc: 1, code_bytes: vec![], current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: hash_set(),
        }, Func { // top level form
            argc: 0, localc: 0, code_bytes: vec![], current_depth: 0,
            loop_point: 0, recur_targets: vec![], loop_terminals: vec![],
            try_point: 0, try_pinned: hash_set(),
        }],
        curr_func: 1,
    });
    let captured = hash_map();
    let res = comp(&mut ctx, form, notes, &captured, hash_map(),
                   hash_set(), hash_set(), vector());
    unimplemented!()
}

// Context:
// -signatures
// -imports
// -globals
// -vars
// -constant_data
// -vtable
// -funcs

// Func:
// -argc
// -localc
// -code_bytes
// -loop_point
// -recur_targets
// -loop_terminals
// -try_point
// -try_pinned

// captured {a 7, b 5}
// named    {c 2, d 5, e 9}  terminal #{c d}
// dormant  #{1 2 3}
// vacant   #{4 7} or vector

// def
// var ns/x
// fn
// do let loop recur
// local x
// if
// call
// primitive calls
// conj nil?
// constants :kw "str" \z true nil 4 1.5
// agg literal [] {} #{}

pub fn comp(ctx: &mut Context, form: &Value, notes: &Value, captured: &Value,
            named: Value, terminal: Value, dormant: Value, vacant: Value) -> Value {
    // symbol? ns or plain?
    // non-empty agg? vec set map? list?
    // first in list, symbol? ns or plain? special form? else call
    // constant? agg literal?
    unimplemented!()
}
pub fn comp_local(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_var(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_def(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_fn(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_if(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_do(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_let(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_loop(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_recur(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_quote(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}
pub fn comp_call(ctx: &mut Context, locals: &Value, form: &Value, notes: &Value) -> Value {
    unimplemented!()
}

