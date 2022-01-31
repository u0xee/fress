// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use memory::unit::Unit;
use ::{Value, hash_map};
use ::{read};

// ns-dependent name resolution
// support for def,
//  interning new var (possibly replacing a refer) or
//  redefining previously interned var
// support for in-ns, refer, alias

//  symbol? ns map of interned/referred. Now qualified symbol (or not found, error)
//  sym/bol? ns alias mapping? Now qualified (with a real ns, not an alias)
//  Ensure qualified symbol names an existing var
// if failed, scrape candidate globals and report to user
pub fn resolve(sym: &Value) -> Result<Value, Value> {
    if sym.has_namespace() {
        // TODO respect ns aliases
        if get_vars().contains(sym) {
            Ok(sym.split_out())
        } else {
            let msg = format!("Qualified symbol {} does not exist", sym);
            let ret = Value::from(msg.as_str());
            Err(ret)
        }
    } else {
        let full = get_names().get(get_curr_ns())
            .get(&get_statics().key_mapped).get(sym).split_out();
        if full.is_nil() {
            // TODO scrape candidate names
            let msg = format!("Plain symbol {} does not exist", sym);
            let ret = Value::from(msg.as_str());
            Err(ret)
        } else {
            Ok(full)
        }
    }
}

use std::cell::Cell;
thread_local! {
    pub static CURR_NS: Cell<Unit> = Cell::new(Unit { word: 0});
    pub static NAMES: Cell<Unit> = Cell::new(Unit { word: 0});
    pub static VARS: Cell<Unit> = Cell::new(Unit { word: 0});
}

static MAPPED: &'static str = "\
{conj fress/conj,
 nil? fress/nil?,
 print console/print}
";
static INIT_VARS: &'static str = "\
{fress/conj nil,
 fress/nil? nil,
 console/print nil}
";

use eval::get_statics;

pub fn init() {
    let fress_ns = {
        let m = read(MAPPED);
        hash_map().assoc(get_statics().key_mapped.split_out(), m)
            .assoc(get_statics().key_alias.split_out(), hash_map())
    };
    let names = hash_map().assoc(get_statics().sym_fress.split_out(), fress_ns);
    let curr_ns = get_statics().sym_fress.split_out();
    CURR_NS.with(|c| c.set(curr_ns._consume().unit()));
    NAMES.with(|c| c.set(names._consume().unit()));
    VARS.with(|c| c.set(read(INIT_VARS)._consume().unit()));
}

pub fn get_curr_ns() -> &'static Value {
    let x = CURR_NS.with(|c| c.as_ptr());
    unsafe { &*(x as usize as *const Value) }
}
pub fn get_names() -> &'static Value {
    let x = NAMES.with(|c| c.as_ptr());
    unsafe { &*(x as usize as *const Value) }
}
pub fn get_vars() -> &'static Value {
    let x = VARS.with(|c| c.as_ptr());
    unsafe { &*(x as usize as *const Value) }
}

// NS
// *ns*
// {fress {:mapped {conj fress/conj,
//                  print console/print},
//         :alias {alg algorithms.core},
//  user ...}
// Vars
// {console/print #'var,
//  fress/conj #'var,
//  ...}

// def conj v
// - Redef?
// - take over refer?
#[no_mangle]
pub extern fn def() {}
// refer conj user/conj
#[no_mangle]
pub extern fn refer() {}
// alias g graph
#[no_mangle]
pub extern fn alias() {}
// var-for fress/conj
#[no_mangle]
pub extern fn var() {}
// in-ns


/// Var dispatch.
pub struct Var { }

impl Var {
    pub fn new() -> Unit {
        unimplemented!()
    }
}



