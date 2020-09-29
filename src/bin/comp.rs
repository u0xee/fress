// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;

use fress::Value;
use fress::eval;
use std::fs::File;
use std::io::Write;

fn n() {
    let s = "(let [a 7]\
                     (fn ([x]\
                          (conj x a))\
                         ([y & z]\
                          (conj a a))))";
    let form = fress::read(s).unwrap();
    eval::init();
    let structured = eval::structure::structure(&form).unwrap();

    use fress::meta;
    meta::do_print_meta();
    println!("Structured: {}", structured);
    meta::end_print_meta();
    unimplemented!();

    let ctx = eval::compile::compile_top_level(&structured);
    eval::compile::show_context(&ctx);
    let module_bytes = eval::assemble::wasm_module(&ctx);
    let mut file = File::create("out.wasm").unwrap();
    file.write_all(&module_bytes).expect("Failed to write out.wasm");
    // TODO eval::uninit();
}

fn main() {
    use fress::memory::segment;
    let (new_a, free_a) = segment::new_free_counts();
    use std::panic;
    let _r = panic::catch_unwind(|| {
        n();
    });
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}

