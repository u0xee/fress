// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, hash_map, hash_set, list, vector, nil, right_into};
use transduce::Process;
use handle::Handle;
use keyword;
use symbol;

pub struct Sum {
    pub defining: Handle,
}

use super::get_statics;
pub fn structure(v: &Value) -> Result<Value, String> {
    let mut sum = Sum { defining: hash_map()._consume() };
    let locals = hash_map();
    let r = res(&mut sum, v, &locals, nil())?;
    // TODO attach defining set, as meta, to top level form
    Ok(r)
}
pub fn res(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    if v.is_symbol() {
        let loc = locals.get(v);
        if !loc.is_nil() {
            let ret = v.split_out().assoc_meta(get_statics().refers_to.split_out(), loc.split_out());
            return Ok(ret)
        }
        let defined = unsafe { *sum.defining.get(v._handle()) };
        if !defined.is_nil() {
            unimplemented!()
        }
        use eval::var;
        let r = var::resolve(v);
        match r {
            Err(s) => {
                // if failed, scrape candidate locals and report to user
                return Err(s.to_string())
            },
            Ok(var_name) => {
                let ret = var_name.assoc_meta(get_statics().resolved_from.split_out(), v.split_out());
                return Ok(ret)
            },
        }
    }
    if v.is_aggregate() && !v.is_empty() {
        if v.is_list() {
            let first = v.peek();
            if first.is_symbol() {
                // TODO compare with static syms
                let s = format!("{}", first);
                if &s == "def" { return res_def(sum, v, locals, tail_of) }
                if &s == "fn" { return res_fn(sum, v, locals, tail_of) }
                if &s == "if" { return res_if(sum, v, locals, tail_of) }
                if &s == "do" { return res_do(sum, v, locals, tail_of) }
                if &s == "let" { return res_let(sum, v, locals, tail_of) }
                if &s == "loop" { return res_loop(sum, v, locals, tail_of) }
                if &s == "recur" { return res_recur(sum, v, locals, tail_of) }
                if &s == "quote" { return res_quote(sum, v, locals, tail_of) }
                if &s == "template" { return res_quote(sum, v, locals, tail_of) }
                // TODO macro call
                return res_call(sum, v, locals, tail_of)
            }
            // (:hat a)
            return res_call(sum, v, locals, tail_of)
        }
        if v.is_vector() { return res_vector(sum, v, locals, tail_of) }
        if v.is_set() { return res_set(sum, v, locals, tail_of) }
        if v.is_map() { return res_map(sum, v, locals, tail_of) }
    }
    return Ok(v.split_out())
}
pub fn res_call(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    let (resolved, locals_used) = res_body(sum, v, locals, nil())?;
    Ok(resolved.assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn res_vector(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    let (resolved, locals_used) = res_body(sum, v, locals, nil())?;
    Ok(resolved.assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn res_set(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    let as_vec = right_into(vector(), v.split_out());
    let (resolved_vec, locals_used) = res_body(sum, &as_vec, locals, nil())?;
    let m = v.meta().split_out().assoc(get_statics().local_use.split_out(), locals_used);
    Ok(right_into(hash_set(), resolved_vec).with_meta(m))
}
pub fn res_map(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    let mut a = hash_map().with_meta(v.meta().split_out());
    let mut locals_used = hash_map();
    let kv = right_into(vector(), set_of_keys(v.split_out()));
    let ct = kv.count();
    for i in 0..ct {
        let k = kv.nth(i);
        let resolved_k = res(sum, k, locals, nil())?;
        locals_used = count_form(sum, locals_used, &resolved_k);
        let resolved_v = res(sum, v.get(k), locals, nil())?;
        locals_used = count_form(sum, locals_used, &resolved_v);
        a = a.assoc(resolved_k.split_out(), resolved_v.split_out());
    }
    let local_use_key = get_statics().local_use.split_out();
    Ok(a.assoc_meta(local_use_key, locals_used))
}
pub fn res_def(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    // (def a _)
    let mut a = v.split_out();
    let ct = a.count();
    assert!(ct == 2 || ct == 3);
    let name = a.nth(1);
    assert!(valid_name(name));
    sum.defining = sum.defining.assoc(name.split_out()._consume(), name.split_out()._consume());
    if ct == 3 {
        let resolved = res(sum, a.nth(2), locals, nil())?;
        let locals_used = count_form(sum, hash_map(), &resolved);
        a = a.assoc(2.into(), resolved);
        Ok(a.assoc_meta(get_statics().local_use.split_out(), locals_used))
    } else {
        Ok(a)
    }
}
pub fn res_do(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    let (body, do_sym) = v.split_out().pop();
    let (rbody, locals_used) = res_body(sum, &body, locals, tail_of)?;
    Ok(rbody.conj(do_sym).assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn res_body(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<(Value, Value), String> {
    let mut a = v.split_out();
    let mut locals_used = hash_map(); // {a 3, b 1, d 1}
    let ct = a.count();
    for i in 0..ct {
        let t = if i == (ct - 1) { tail_of.split_out() } else { nil() };
        let resolved = res(sum, a.nth(i), locals, t)?;
        locals_used = count_form(sum, locals_used, &resolved);
        a = a.assoc(i.into(), resolved);
    }
    Ok((a, locals_used))
}
pub fn res_fn(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    // (fn name? [x y] _ _ _)
    // (fn name? ([x y] _ _ _)
    //           ([x y z] _ _ _))
    assert!(v.count() > 1);
    let (fn_sym, name, mut bodies) = {
        let (a, fn_sym) = v.split_out().pop();
        let (b, name) = if a.peek().is_symbol() { a.pop() } else { (a, nil()) };
        let bodies = if b.peek().is_vector() { vector().conj(b) } else { b };
        (fn_sym, name, bodies)
    };
    let locals_plus = if name.is_nil() { locals.split_out() } else {
        assert!(valid_name(&name));
        locals.split_out().assoc(name.split_out(), name.split_out())
    };
    let mut arity_bitmap = 0u32;
    let mut vararg_arity = 0u32;
    let mut arity_to_idx = hash_map();
    let mut locals_used = hash_map();
    let ct = bodies.count();
    for i in 0..ct {
        let body = bodies.nth(i);
        assert!(body.is_list() && body.peek().is_vector());
        let (bod, used, arg_ct, var_arg) = res_fn_body(sum, body, &locals_plus)?;
        assert!(!has_bit(arity_bitmap, arg_ct));
        arity_bitmap = set_bit(arity_bitmap, arg_ct);
        if var_arg {
            assert_eq!(vararg_arity, 0);
            vararg_arity = arg_ct;
        }
        arity_to_idx = arity_to_idx.assoc(arg_ct.into(), i.into());
        locals_used = merge_counts(locals_used, used);
        bodies = bodies.assoc(i.into(), bod);
    }
    if vararg_arity != 0 {
        assert_eq!(vararg_arity, max_arity(arity_bitmap));
    }
    let locals_sans = locals_used.dissoc(&name);
    let captures = right_into(vector(), set_of_keys(locals_sans.split_out()));
    let b = if bodies.is_list() { bodies } else { bodies.pop().1 };
    let a = if name.is_nil() { b } else { b.conj(name) };
    let resolved = a.conj(fn_sym);
    let res = resolved
        .assoc_meta(get_statics().local_use.split_out(), locals_sans)
        .assoc_meta(get_statics().arity_bitmap.split_out(), arity_bitmap.into())
        .assoc_meta(get_statics().arity_to_idx.split_out(), arity_to_idx)
        .assoc_meta(get_statics().vararg.split_out(), (vararg_arity != 0).into())
        .assoc_meta(get_statics().captures.split_out(), captures);
    Ok(res)
}
pub fn has_bit(bitmap: u32, idx: u32) -> bool { bitmap & (1u32 << idx) != 0 }
pub fn set_bit(bitmap: u32, idx: u32) -> u32 { bitmap | (1u32 << idx) }
pub fn max_arity(bitmap: u32) -> u32 { 32 - 1 - bitmap.leading_zeros() }
pub fn args_arity(args_form: &Value) -> (u32, bool) {
    let ct = args_form.count();
    if ct > 1 && args_form.nth(ct - 2) == &get_statics().sym_amp {
        (ct - 1, true)
    } else { (ct, false) }
}
pub fn res_fn_body(sum: &mut Sum, fn_body: &Value, locals: &Value)
                   -> Result<(Value, Value, u32, bool), String> {
    // ([x y] _ _ _)
    // ([x y & z] _)
    let (body, args) = fn_body.split_out().pop();
    let (arg_ct, var_arg) = args_arity(&args);
    let args_locals = args_locals(&args, var_arg)?;
    let locals_plus = right_into(locals.split_out(), args_locals);
    let recur_target = args.split_out();
    let (bod, locals_used) = res_body(sum, &body, &locals_plus, recur_target)?;
    let (locals_sans, args_meta) = count_args_bindings(locals_used, args, var_arg);
    let resolved = bod.conj(args_meta);

    let recur_sym = get_statics().sym_recur.split_out();
    if locals_sans.contains(&recur_sym) {
        // TODO add recur to meta
        //let locals_sans = locals_used.dissoc(&recur_sym);
        unimplemented!()
    }
    Ok((resolved, locals_sans, arg_ct, var_arg))
}
pub fn args_locals(args_form: &Value, is_var_args: bool) -> Result<Value, String> {
    let mut locals = hash_map();
    let ct = args_form.count();
    for i in 0..ct {
        if is_var_args && i == ct - 2 { continue; }
        let loc = locals_from(args_form.nth(i))?;
        locals = merge_disjoint_locals(locals, loc)?;
    }
    Ok(locals)
}
pub fn count_args_bindings(locals_used: Value, args_form: Value, is_var_args: bool) -> (Value, Value) {
    let mut locals = locals_used;
    let mut a = args_form;
    let ct = a.count();
    for i in 0..ct {
        if is_var_args && i == ct - 2 { continue; }
        let (loc, binding_form) = count_bindings(locals, a.nth(i));
        locals = loc;
        a = a.assoc(i.into(), binding_form);
    }
    (locals, a)
}
pub fn res_if(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    // (if test then _)
    let mut a = v.split_out();
    let ct = a.count();
    assert!(ct == 3 || ct == 4); // TODO return Err with advice
    let mut locals_used = hash_map();

    let resolved_test = res(sum, a.nth(1), locals, nil())?;
    locals_used = count_form(sum, locals_used, &resolved_test);
    let resolved_then = res(sum, a.nth(2), locals, tail_of.split_out())?;
    locals_used = count_form(sum, locals_used, &resolved_then);
    if ct == 4 {
        let resolved_else = res(sum, a.nth(3), locals, tail_of)?;
        // TODO if in loop tail, and exactly one branch returns "recur", flag in notes as loop exit branch
        //  xor resolved_then meta recur with resolved_else
        locals_used = count_form(sum, locals_used, &resolved_else);
        a = a.assoc(1.into(), resolved_test);
        a = a.assoc(2.into(), resolved_then);
        a = a.assoc(3.into(), resolved_else);
    } else {
        a = a.assoc(1.into(), resolved_test);
        a = a.assoc(2.into(), resolved_then);
    }
    Ok(a.assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn pop2(v: Value) -> (Value, Value, Value) {
    let (rest, first) = v.pop();
    let (rest, second) = rest.pop();
    (first, second, rest)
}
pub fn res_let(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    assert!(v.count() > 1);
    let b = v.nth(1);
    assert!(b.is_vector() && (b.count() & 0x1 == 0)); // TODO return Err("example let")
    let (let_sym, bindings, body) = pop2(v.split_out());
    let (bind, bod, locals_used) = let_rec(sum, &bindings, &body, 0, locals, tail_of)?;
    let resolved = bod.conj(bind).conj(let_sym);
    Ok(resolved.assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn let_rec(sum: &mut Sum, bindings: &Value, body: &Value, bind_idx: u32,
               locals: &Value, tail_of: Value) -> Result<(Value, Value, Value), String> {
    let bind_ct = bindings.count() >> 1;
    assert!(bind_idx <= bind_ct);
    if bind_idx == bind_ct { // body
        let (rbody, locals_used) = res_body(sum, body, locals, tail_of)?;
        Ok((bindings.split_out(), rbody, locals_used))
    } else { // binding
        let idx = bind_idx << 1;
        let resolved_exp = res(sum, bindings.nth(idx + 1), locals, nil())?;
        let binding_form = bindings.nth(idx);
        let locals_with_bindings = with_bindings(locals, binding_form)?;
        let (bind, bod, locals_used) =
            let_rec(sum, bindings, body, bind_idx + 1, &locals_with_bindings, tail_of)?;
        let (locals_sans, binding_form_meta) =
            count_bindings(locals_used, binding_form);
        let locals_exp = count_form(sum, locals_sans, &resolved_exp);
        let b = bind.assoc(idx.into(), binding_form_meta)
            .assoc((idx + 1).into(), resolved_exp);
        Ok((b, bod, locals_exp))
    }
}
pub fn with_bindings(locals: &Value, binding_form: &Value) -> Result<Value, String> {
    let loc = locals_from(binding_form)?;
    Ok(right_into(locals.split_out(), loc))
}
pub fn locals_from(binding_form: &Value) -> Result<Value, String> {
    // to "locals" map from all the symbols in the binding form
    // (also names in def) validate names, no namespace, not one of the special form names
    if valid_name(binding_form) {
        return Ok(hash_map().assoc(binding_form.split_out(), binding_form.split_out()))
    }
    // [a b c]  [a b :as v]
    // {a :a, b :bit-width, :as m}
    // {:person/keys [name age], :or {age 50}}
    unimplemented!()
}
pub fn valid_name(s: &Value) -> bool {
    // TODO check not special form name
    s.is_symbol() && !s.has_namespace()
}
pub fn count_bindings(locals_used: Value, binding_form: &Value) -> (Value, Value) {
    if binding_form.is_symbol() {
        let entry = locals_used.get(binding_form).split_out();
        let ct = if entry.is_nil() { 0.into() } else { entry };
        let b = binding_form.split_out().assoc_meta(get_statics().forms_using.split_out(), ct);
        return (locals_used.dissoc(binding_form), b)
    }
    // used sans bindings, binding_form with meta attached to symbols
    unimplemented!()
}

pub fn res_loop(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    assert!(v.count() > 1);
    let b = v.nth(1);
    assert!(b.is_vector() && (b.count() & 0x1 == 0));
    let (loop_sym, bindings, body) = pop2(v.split_out());
    let recur_target = gather_loop_bindings(&bindings);
    let (bind, bod, locals_used) =
        let_rec(sum, &bindings, &body, 0, locals, recur_target)?;
    let recur_sym = get_statics().sym_recur.split_out();
    assert!(locals_used.contains(&recur_sym));
    let resolved = bod.conj(bind).conj(loop_sym);
    let locals_sans = locals_used.dissoc(&recur_sym);
    Ok(resolved.assoc_meta(get_statics().local_use.split_out(), locals_sans))
}
pub fn gather_loop_bindings(b: &Value) -> Value {
    let mut bindings = vector();
    let binding_count = b.count() >> 1;
    for i in 0..binding_count {
        bindings = bindings.conj(b.nth(i << 1).split_out());
    }
    bindings
}
pub fn res_recur(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    // (recur 1 2)
    if tail_of.is_nil() { return Err(format!("Recur not in tail position!")); }
    if tail_of.count() != (v.count() - 1) {
        return Err(format!("Recur does not match, in number of arguments."));
    }
    let (resolved, locals_used) = {
        let (exps, recur_sym) = v.split_out().pop();
        let (rexps, locals_used) = res_body(sum, &exps, locals, nil())?;
        (rexps.conj(recur_sym.split_out()), locals_used.assoc(recur_sym, 1.into()))
    };
    Ok(resolved.assoc_meta(get_statics().local_use.split_out(), locals_used))
}
pub fn res_quote(sum: &mut Sum, v: &Value, locals: &Value, tail_of: Value) -> Result<Value, String> {
    // (quote (a b c))
    let ct = v.count();
    assert_eq!(ct, 2); // TODO return Err(how to use)
    Ok(v.split_out())
}

pub fn count_form(sum: &mut Sum, local_counts: Value, subform: &Value) -> Value {
    if subform.is_symbol() && !subform.has_namespace() {
        return merge_one(local_counts, subform.split_out())
    }
    let form_counts = subform.meta().get(&get_statics().local_use);
    if form_counts.is_nil() {
        local_counts
    } else {
        merge_counts(local_counts, form_counts.split_out())
    }
}
pub fn merge_counts(locals_used: Value, used: Value) -> Value {
    use handle::Handle;
    struct Reduce {
        r: Handle,
    }
    impl Process for Reduce {
        fn inges_kv(&mut self, _stack: &mut [Box<dyn Process>], k: &Value, v: &Value) -> Option<Value> {
            self.r = merge_one(self.r.value(), k.split_out())._consume();
            None
        }
        fn last_call(&mut self, _stack: &mut [Box<dyn Process>]) -> Value { self.r.value() }
    }
    let mut stack: Box<dyn Process> = Box::new(Reduce { r: locals_used._consume() });
    use std::slice::from_mut;
    used.reduce(from_mut(&mut stack))
}

pub fn merge_one(counts: Value, s: Value) -> Value {
    let new_v = {
        let v = counts.get(&s);
        if v.is_nil() { Value::from(1) } else { v.split_out().inc() }
    };
    counts.assoc(s, new_v)
}

pub fn merge_disjoint_locals(a: Value, b: Value) -> Result<Value, String> {
    struct Merge { c: Handle, }
    impl Process for Merge {
        fn ingest_kv(&mut self, stack: &mut [Box<dyn Process>], k: Value, v: Value)
                     -> Option<Value> {
            if self.c.contains(k._handle()) {
                // if c.contains(k) && k is not _
                let c = self.c.value();
                let msg = vector().conj("Duplicate locals:".into())
                    .conj(c.get(&k).split_out()).conj(v);
                Some(msg)
            } else {
                self.c = self.c.assoc(k._consume(), v._consume());
                None
            }
        }
        fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value { self.c.value() }
    }
    let mut stack: Box<dyn Process> = Box::new(Merge { c: a._consume() });
    use std::slice::from_mut;
    let r = b.reduce(from_mut(&mut stack));
    if r.is_vector() {
        Err(format!("{:?}", r))
    } else {
        Ok(r)
    }
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
    struct Collect { c: Handle }
    impl Process for Collect {
        fn ingest   (&mut self, stack: &mut [Box<dyn Process>], v: Value) -> Option<Value> {
            self.c = self.c.conj(Handle::from(v));
            None
        }
        fn ingest_kv(&mut self, stack: &mut [Box<dyn Process>], k: Value, v: Value)
                     -> Option<Value> {
            self.c = self.c.assoc(Handle::from(k), Handle::from(v));
            None
        }
        fn last_call(&mut self, stack: &mut [Box<dyn Process>]) -> Value { self.c.value() }
    }
    Box::new(Collect { c: Handle::from(col) })
}

