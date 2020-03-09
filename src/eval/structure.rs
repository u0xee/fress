// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, hash_map, hash_set, list, vector, nil, right_into};
use transduce::Process;

pub fn structure(v: Value, globals: &Value) -> Result<(Value, Value), String> {
    let locals = hash_set();
    let r = res(v, globals, &locals, nil());
    match r {
        Err(s) => { return Err(s) },
        Ok((resolved, locals_used, notes)) => {
            if locals_used.is_set() && !locals_used.is_empty() {
                panic!("Not in local context, locals_used: {}", locals_used);
            }
            /*if !notes.is_nil() {
                let notes_keys = right_into(vector(), set_of_keys(notes.split_out()));
                for i in 0..(notes_keys.count()) {
                    let e = notes_keys.nth(i);
                    println!("{} {}", e, notes.get(&e));
                }
            }*/
            return Ok((resolved, notes))
        },
    }
}

// TODO propagate recur like a local use
// TODO on loop-tail recur and non-recur branches, mark in notes as exiting loop path
// TODO check arity collisions in multi-body fns
// TODO normalize things like loop w/o recur -> let, recur targeting fn args, destructuring bindings

pub fn resolve_symbol(s: &Value, globals: &Value, locals: &Value) -> Option<Value> {
    if s.is_symbol() {
        log!("Resolving symbol {}", s);
        if s.has_namespace() {
            // check exists in globals
            Some(s.split_out())
        } else {
            if locals.contains(s) {
                log!("Found in locals {}", s);
                Some(s.split_out())
            } else {
                // like *ns* *macros*, *aliases*
                let r = &globals[s];
                if r.is_symbol() {
                    log!("Found in globals {}", s);
                    Some(r.split_out())
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

pub fn res(v: Value, globals: &Value, locals: &Value, tail_of: Value)
           -> Result<(Value, Value, Value), String> {
    if v.is_symbol() {
        if let Some(resolved) = resolve_symbol(&v, globals, locals) {
            if resolved.has_namespace() {
                return Ok((resolved, nil(), nil()))
            } else {
                assert_eq!(v, resolved);
                return Ok((resolved, v, nil()))
            }
        } else {
            return Err(format!("Cannot resolve symbol {} !", v))
        }
    }
    if v.is_aggregate() && !v.is_empty() {
        if v.is_list() {
            let first = v.peek();
            if first.is_symbol() {
                let s = format!("{}", first);
                if &s == "def" { return res_def(v, globals, locals, tail_of) }
                if &s == "fn" { return res_fn(v, globals, locals, tail_of) }
                if &s == "if" { return res_if(v, globals, locals, tail_of) }
                if &s == "do" { return res_do(v, globals, locals, tail_of) }
                if &s == "let" { return res_let(v, globals, locals, tail_of) }
                if &s == "loop" { return res_loop(v, globals, locals, tail_of) }
                if &s == "recur" { return res_recur(v, globals, locals, tail_of) }
                if &s == "quote" { return res_quote(v, globals, locals, tail_of) }

                if let Some(rs) = resolve_symbol(first, globals, locals) {
                    if rs.has_namespace() && is_macro(&rs, globals) {
                        unimplemented!("macro invocation")
                    } else {
                        return res_call(v, globals, locals, tail_of)
                    }
                } else {
                    return Err(format!("Cannot resolve symbol {} !", first))
                }
            }
            // (:hat a)
            return res_call(v, globals, locals, tail_of)
        }
        if v.is_vector() { return res_vector(v, globals, locals, tail_of) }
        if v.is_set() { return res_set(v, globals, locals, tail_of) }
        if v.is_map() { return res_map(v, globals, locals, tail_of) }
    }
    return Ok((v, nil(), nil()))
}

pub fn is_macro(s: &Value, globals: &Value) -> bool {
    group!("structure is_macro? on symbol {}", s);
    let ret = {
        let macros = read("*macros* ").unwrap();
        let macro_set = globals.get(&macros);
        if macro_set.is_nil() { false } else { macro_set.contains(s) }
    };
    group_end!();
    ret
}

pub fn res_call(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                -> Result<(Value, Value, Value), String> {
    // (def cat (fn [x] (fn [y] (+ x y))))
    group!("Resolving a call");
    let mut aa = a.split_out();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let ct = a.count();
    for i in 0..ct {
        let e = a.nth(i);
        let r = res(e.split_out(), globals, locals, nil());
        match r {
            Ok((resolved, used, nt)) => {
                aa = aa.assoc(i.into(), resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() { notes = right_into(notes, nt); }
            },
            Err(s) => { return Err(s) },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    group_end!();
    Ok((aa, locals_set, last_note))
}

pub fn res_vector(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                  -> Result<(Value, Value, Value), String> {
    group!("Resolving a vector");
    let mut aa = a.split_out();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let ct = a.count();
    for i in 0..ct {
        let e = a.nth(i);
        let r = res(e.split_out(), globals, locals, nil());
        match r {
            Ok((resolved, used, nt)) => {
                aa = aa.assoc(i.into(), resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() {
                    notes = right_into(notes, nt);
                }
            },
            Err(s) => {
                group_end!();
                return Err(s)
            },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    group_end!();
    Ok((aa, locals_set, last_note))
}

pub fn res_set(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                  -> Result<(Value, Value, Value), String> {
    let mut aa = a.empty();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let av = right_into(vector(), a);
    let ct = av.count();
    for i in 0..ct {
        let e = av.nth(i);
        let r = res(e.split_out(), globals, locals, nil());
        match r {
            Ok((resolved, used, nt)) => {
                aa = aa.conj(resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() {
                    notes = right_into(notes, nt);
                }
            },
            Err(s) => { return Err(s) },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    Ok((aa, locals_set, last_note))
}

pub fn res_map(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                  -> Result<(Value, Value, Value), String> {
    let mut aa = a.empty();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let kv = right_into(vector(), set_of_keys(a.split_out()));
    let ct = kv.count();
    for i in 0..ct {
        let k = kv.nth(i);
        let v = a.get(k);
        let rk = res(k.split_out(), globals, locals, nil());
        let resolved_key = match rk {
            Ok((resolved, used, nt)) => {
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() {
                    notes = right_into(notes, nt);
                }
                resolved
            },
            Err(s) => { return Err(s) },
        };
        let rv = res(v.split_out(), globals, locals, nil());
        match rv {
            Ok((resolved_val, used, nt)) => {
                aa = aa.assoc(resolved_key, resolved_val);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() {
                    notes = right_into(notes, nt);
                }
            },
            Err(s) => { return Err(s) },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    Ok((aa, locals_set, last_note))
}

pub fn res_def(a: Value, globals: &Value, locals: &Value, tail_of: Value)
               -> Result<(Value, Value, Value), String> {
    // (def a _)
    group!("Resolving a def");
    let ct = a.count();
    assert!(ct == 2 || ct == 3);
    let name = a.nth(1);
    assert!(name.is_symbol() && !name.has_namespace());
    let ret = if ct == 3 {
        // add name to globals map
        let r = res(a.nth(2).split_out(), globals, locals, nil());
        match r {
            Ok((resolved, locals_used, notes)) => {
                Ok((a.assoc(2.into(), resolved), locals_used, notes))
            },
            Err(s) => { Err(s) },
        }
    } else {
        Ok((a, nil(), nil()))
    };
    group_end!();
    ret
}

pub fn res_do(a: Value, globals: &Value, locals: &Value, tail_of: Value)
              -> Result<(Value, Value, Value), String> {
    // (do _ _)
    group!("Resolving do");
    let mut aa = a.split_out();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let ct = a.count();
    for i in 1..ct {
        let e = a.nth(i);
        let t = if i == (ct - 1) { tail_of.split_out() } else { nil() };
        let r = res(e.split_out(), globals, locals, t);
        match r {
            Ok((resolved, used, nt)) => {
                aa = aa.assoc(i.into(), resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() { notes = right_into(notes, nt); }
            },
            Err(s) => {
                group_end!();
                return Err(s)
            },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    group_end!();
    Ok((aa, locals_set, last_note))
}

pub fn res_fn(a: Value, globals: &Value, locals: &Value, tail_of: Value)
              -> Result<(Value, Value, Value), String> {
    // (fn name? [x y] _ _ _)
    // (fn name? ([x y] _ _ _)
    //           ([x y z] _ _ _))
    group!("Resolving fn");
    let (body, fn_sym, name, locals_with_name) = {
        let (a2, fn_sym) = a.pop();
        if a2.peek().is_symbol() {
            let (b, name) = a2.pop();
            (b, fn_sym, name.split_out(), locals.split_out().conj(name))
        } else {
            (a2, fn_sym, nil(), locals.split_out())
        }
    };
    assert!(name.is_nil() || (name.is_symbol() && !name.has_namespace()));
    let bodies = if body.peek().is_vector() { list().conj(body) } else { body };

    let mut locals_set = hash_set();
    let mut notes = hash_map();
    let mut bodies2 = bodies.split_out();

    let ct = bodies.count();
    for i in 0..ct {
        let b = bodies.nth(i);
        let r = res_fn_body(b.split_out(), globals, &locals_with_name);
        match r {
            Err(s) => {
                group_end!();
                return Err(s)
            },
            Ok((resolved, used, nt)) => {
                bodies2 = bodies2.assoc(i.into(), resolved);
                locals_set = right_into(locals_set, used);
                if !nt.is_nil() { notes = right_into(notes, nt); }
            }
        }
    }
    let resolved_body = if bodies2.count() == 1 {
        let (_empty_list, b) = bodies2.pop();
        b
    } else {
        bodies2
    };
    let resolved = if name.is_nil() { resolved_body.conj(fn_sym) } else {
        resolved_body.conj(name.split_out()).conj(fn_sym)
    };
    if !name.is_nil() {
        locals_set = locals_set.dissoc(&name);
    }
    let last_note = notes.assoc(resolved.split_out(), locals_set.split_out());
    group_end!();
    Ok((resolved, locals_set, last_note))
}

pub fn res_fn_body(body: Value, globals: &Value, locals: &Value)
                   -> Result<(Value, Value, Value), String> {
    // ([x y] _ _ _)
    group!("Resolving fn body");
    let (exprs, args) = body.pop();
    for i in 0..(args.count()) {
        let x = args.nth(i);
        assert!(x.is_symbol() && !x.has_namespace())
    }
    let locals_with_args = right_into(locals.split_out(), args.split_out());
    let mut locals_used = hash_map();
    let mut notes = hash_map();
    let mut exprs2 = exprs.split_out();

    let ct = exprs.count();
    for i in 0..ct {
        let e = exprs.nth(i);
        let t = if i == (ct - 1) { args.split_out() } else { nil() };
        let r = res(e.split_out(), globals, &locals_with_args, t);
        match r {
            Ok((resolved, used, nt)) => {
                exprs2 = exprs2.assoc(i.into(), resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() { notes = right_into(notes, nt); }
            },
            Err(s) => {
                group_end!();
                return Err(s)
            },
        }
    }

    let mut args_use = vector();
    let args_ct = args.count();
    for i in 0..args_ct {
        let arg = args.nth(i).split_out();
        let arg_use = {
            let arg_use = locals_used.get(&arg).split_out();
            if arg_use.is_nil() { Value::from(0) } else {
                locals_used = locals_used.dissoc(&arg);
                arg_use
            }
        };
        args_use = args_use.conj(arg).conj(arg_use);
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let resolved_body = exprs2.conj(args);
    let last_note = notes.assoc(resolved_body.split_out(),
                                vector().conj(args_use).conj(locals_used));
    // TODO move recur to notes
    group_end!();
    Ok((resolved_body, locals_set, last_note))
}

pub fn res_if(a: Value, globals: &Value, locals: &Value, tail_of: Value)
              -> Result<(Value, Value, Value), String> {
    // (if test then _)
    let ct = a.count();
    assert!(ct == 3 || ct == 4); // return Err

    let mut aa = a.split_out();
    let mut locals_used = hash_map();
    let mut notes = hash_map();

    let te = a.nth(1);
    let r = res(te.split_out(), globals, locals, nil());
    match r {
        Ok((resolved, used, nt)) => {
            aa = aa.assoc(1.into(), resolved);
            locals_used = merge_counts(locals_used, used);
            if !nt.is_nil() { notes = right_into(notes, nt); }
        },
        Err(s) => { return Err(s) },
    }

    let th = a.nth(2);
    let r = res(th.split_out(), globals, locals, tail_of.split_out());
    match r {
        Ok((resolved, used, nt)) => {
            aa = aa.assoc(2.into(), resolved);
            locals_used = merge_counts(locals_used, used);
            if !nt.is_nil() { notes = right_into(notes, nt); }
        },
        Err(s) => { return Err(s) },
    }

    // if in loop tail, and exactly one branch returns "recur", flag in notes as loop exit branch
    if ct == 4 {
        let el = a.nth(3);
        let r = res(el.split_out(), globals, locals, tail_of);
        match r {
            Ok((resolved, used, nt)) => {
                aa = aa.assoc(3.into(), resolved);
                locals_used = merge_counts(locals_used, used);
                if !nt.is_nil() { notes = right_into(notes, nt); }
            },
            Err(s) => { return Err(s) },
        }
    }
    let locals_set = set_of_keys(locals_used.split_out());
    let last_note = notes.assoc(aa.split_out(), locals_used);
    Ok((aa, locals_set, last_note))
}

pub fn res_let(a: Value, globals: &Value, locals: &Value, tail_of: Value)
               -> Result<(Value, Value, Value), String> {
    // (let [a 1 b 2]
    //   (+ a b))
    group!("Resolving let");
    assert!(a.count() > 1);
    let b = a.nth(1);
    assert!(b.is_vector() && (b.count() & 0x1 == 0));
    let (resolved, used, bindings_used, notes) = {
        let r = let_rec(a, 0, globals, locals, tail_of);
        match r {
            Err(s) => {
                group_end!();
                return Err(s)
            },
            Ok(x) => x,
        }
    };
    let locals_set = set_of_keys(used.split_out());
    let n = vector().conj(right_into(vector(), bindings_used)).conj(used);
    let last_note = notes.assoc(resolved.split_out(), n);
    group_end!();
    Ok((resolved, locals_set, last_note))
}

pub fn let_rec(a: Value, binding: u32, globals: &Value, locals: &Value, tail_of: Value)
               -> Result<(Value, Value, Value, Value), String> {
    let b = a.nth(1);
    let binding_count = b.count() >> 1;
    assert!(binding <= binding_count);
    if binding == binding_count { // body
        group!("Resolving let body");
        let mut aa = a.split_out();
        let mut locals_used = hash_map();
        let mut notes = hash_map();

        let ct = a.count();
        for i in 2..ct {
            let e = a.nth(i);
            let t = if i == (ct - 1) { tail_of.split_out() } else { nil() };
            let r = res(e.split_out(), globals, locals, t);
            match r {
                Ok((resolved, used, nt)) => {
                    aa = aa.assoc(i.into(), resolved);
                    locals_used = merge_counts(locals_used, used);
                    if !nt.is_nil() { notes = right_into(notes, nt); }
                },
                Err(s) => {
                    group_end!();
                    return Err(s)
                },
            }
        }
        let res = Ok((aa, locals_used, list(), notes));
        group_end!();
        res
    } else { // binding
        let name_idx = binding << 1;
        let name = b.nth(name_idx).split_out();
        assert!(name.is_symbol() && !name.has_namespace());
        let exp = b.nth(name_idx + 1);
        group!("Resolving local binding expression");
        let (resolved_exp, used_exp, nt_exp) = {
            let r = res(exp.split_out(), globals, locals, nil());
            match r {
                Err(s) => {
                    group_end!();
                    return Err(s)
                },
                Ok(x) => { x },
            }
        };
        group_end!();
        let locals_with_name = locals.split_out().conj(name.split_out());
        let (resolved_let, used, used_bindings, nt) = {
            let r = let_rec(a, binding + 1, globals, &locals_with_name, tail_of);
            match r {
                Err(s) => { return Err(s) },
                Ok(x) => { x },
            }
        };
        let (let_form, bindings) = resolved_let.assoc_out(1.into(), nil());
        let resolved_bindings = bindings.assoc((name_idx + 1).into(), resolved_exp);
        let resolved = let_form.assoc(1.into(), resolved_bindings);

        let (name_ct, used_less_name) = {
            let ct = used.get(&name).split_out();
            if ct.is_nil() { (0.into(), used) } else { (ct, used.dissoc(&name)) }
        };
        let used_bindings_with_name = used_bindings.conj(name_ct).conj(name);
        let notes = {
            let notes = if nt.is_nil() { hash_map() } else { nt };
            if nt_exp.is_nil() { notes } else { right_into(notes, nt_exp) }
        };
        let u = merge_counts(used_less_name, used_exp);
        Ok((resolved, u, used_bindings_with_name, notes))
    }
}

pub fn res_loop(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                -> Result<(Value, Value, Value), String> {
    // (loop [a 1, b 2] _ _)
    group!("Resolving a loop");
    assert!(a.count() > 1);
    let b = a.nth(1);
    assert!(b.is_vector() && (b.count() & 0x1 == 0));
    let binding_count = b.count() >> 1;
    let bindings = {
        let mut bindings = vector();
        for i in 0..binding_count {
            bindings = bindings.conj(b.nth(i << 1).split_out());
        }
        bindings
    };
    let r = res_let(a, globals, locals, bindings);
    let ret = match r {
        Ok((resolved, used, nt)) => {
            // TODO move recur to notes
            Ok((resolved, used, nt))
        },
        Err(s) => { Err(s) },
    };
    group_end!();
    ret
}

pub fn res_recur(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                 -> Result<(Value, Value, Value), String> {
    // (recur 1 2)
    group!("Resolving recur");
    let ct = a.count();
    if !tail_of.is_vector() {
        // return Err
        panic!("Recur not in tail position!");
    }
    if tail_of.count() != (ct - 1) {
        // return Err
        panic!("Recur does not match, in number of arguments.");
    }
    let recur_sym = a.nth(0).split_out();
    let r = res_do(a, globals, locals, nil());
    let ret = match r {
        Ok((resolved, used, nt)) => {
            Ok((resolved, used.conj(recur_sym), nt))
        },
        Err(s) => { Err(s) },
    };
    group_end!();
    ret
}

pub fn res_quote(a: Value, globals: &Value, locals: &Value, tail_of: Value)
                 -> Result<(Value, Value, Value), String> {
    // (quote (a b c))
    let ct = a.count();
    assert_eq!(ct, 2);
    Ok((a, nil(), nil()))
}

pub fn set_of_keys(m: Value) -> Value {
    let mut stack = vec!(collect_into(hash_set()), just_keys());
    return m.reduce(&mut stack)
}

pub fn just_keys() -> Box<dyn Process> {
    use transduce::inges;
    struct Keys {}
    impl Process for Keys {
        fn inges_kv (&mut self, stack: &mut [Box<dyn Process>], k: &Value, v: &Value) -> Option<Value> {
            let (_, rest) = stack.split_last_mut().unwrap();
            inges(rest, k)
        }
    }
    Box::new(Keys { })
}

pub fn collect_into(col: Value) -> Box<dyn Process> {
    use handle::Handle;
    struct Collect {
        c: Handle,
    }
    impl Process for Collect {
        fn ingest   (&mut self, stack: &mut [Box<dyn Process>], v: Value) -> Option<Value> {
            self.c = self.c.conj(Handle::from(v));
            None
        }
        fn ingest_kv(&mut self, stack: &mut [Box<dyn Process>], k: Value, v: Value)
                     -> Option<Value> {
            let (c, displaced) = self.c.assoc(Handle::from(k), Handle::from(v));
            displaced.retire();
            self.c = c;
            None
        }
        fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value { self.c.value() }
    }
    Box::new(Collect { c: Handle::from(col) })
}

pub fn merge_counts(locals_used: Value, used: Value) -> Value {
    if used.is_nil() { return locals_used }
    if used.is_symbol() { return merge_one(locals_used, used) }
    if used.is_set() {
        use handle::Handle;
        struct Reduce {
            r: Handle,
        }
        impl Process for Reduce {
            fn ingest   (&mut self, _stack: &mut [Box<dyn Process>], v: Value) -> Option<Value> {
                self.r = Handle::from(merge_one(self.r.value(), v));
                None
            }
            fn last_call(&mut self, _stack: &mut [Box<dyn Process>]) -> Value { self.r.value() }
        }
        let mut stack: Box<dyn Process> = Box::new(Reduce { r: Handle::from(locals_used) });
        use std::slice::from_mut;
        return used.reduce(from_mut(&mut stack))
    }
    unimplemented!("merge_counts with argument: {}", used)
}

pub fn merge_one(counts: Value, s: Value) -> Value {
    let new_v = {
        let v = counts.get(&s);
        if v.is_nil() { Value::from(1) } else { v.split_out().inc() }
    };
    counts.assoc(s, new_v)
}

