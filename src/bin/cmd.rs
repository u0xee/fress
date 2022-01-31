// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;

use fress::Value;
use fress::memory::segment;
use fress::eval;
use fress::edn::reader::EdnRdr;

fn n() {
    let mut reader = EdnRdr::with_buffer_capacity(1 << 10);
    let stop_cmd = fress::read(":repl/quit ");
    println!("Ready!");
    //println!("Ready! stop_cmd 0x{:016X}", stop_cmd._handle().unit().u());
    //println!("Here is stop_cmd {}", stop_cmd);
    loop {
        //io::stdout().flush().ok().expect("Could not flush stdout.");
        let v = read_one_stdin(&mut reader);
        if v == stop_cmd {
            return;
        }
        // let result = eval::eval(v);
        run_cmd(v);
    }
}

fn read_one_stdin(reader: &mut EdnRdr) -> Value {
    //if let Some(v) = reader.read_bytes(b"some\n").unwrap() { return v }
    loop {
        let res = match reader.read_again() {
            Err(m) => {
                println!("{}", m);
                reader.clear_buffer();
                println!("Ready!");
                continue;
            },
            Ok(res) => { res }
        };
        match res {
            Some(v) => { return v },
            None => {
                use std::io::{self, Read};
                let n = io::stdin().read(reader.buffer_wilderness()).unwrap();
                if n == 0 { return fress::read(":repl/quit ") }
                reader.buffer_consume(n);
                /*use std::str;
                let x = str::from_utf8(reader.buf.as_slice()).unwrap();
                println!("Buffer:{}", x);*/
                continue;
            },
        }
    }
}

fn main() {
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

fn run_cmd(v: Value) {
    if !v.is_list() || v.is_empty() {
        println!("■■ {}", v);
        return;
    }
    let cmd = v[0].to_string();
    if cmd.as_bytes()[0] == b':' {
        println!("■■ {}", v[1].get(&v[0]));
        return;
    }
    match cmd.to_string().as_str() {
        "hash" => { println!("■■ 0x{:08X}", v[1].hash()) },
        "type-name" => { println!("■■ {}", v[1].type_name()) },
        "=" => { println!("■■ {}", v[1] == v[2]) },
        "<" => { println!("■■ {}", v[1] < v[2]) },
        ">" => { println!("■■ {}", v[1] > v[2]) },
        "cmp" => { println!("■■ {:?}", v[1].partial_cmp(&v[2]).unwrap()) },
        "count" => { println!("■■ {}", v[1].count()) },
        "meta" => { println!("■■ {}", v[1].meta()) },
        "get" => { println!("■■ {}", v[1].get(&v[2])) },
        "conj" => { println!("■■ {}", v[1].split_out().conj(v[2].split_out())) },
        "assoc" => { println!("■■ {}", v[1].split_out().assoc(v[2].split_out(), v[3].split_out())) },
        "inc" => { println!("■■ {}", v[1].split_out().inc()) },
        "dec" => { println!("■■ {}", v[1].split_out().dec()) },
        "pop" => { let p = v[1].split_out().pop(); println!("■■ {} {}", p.0, p.1) },
        "peek" => { println!("■■ {}", v[1].peek()) },
        "sha3-256" => {
            use fress::hash::keccak::sha3_256_file;
            let s = v[1].to_string();
            let dig = sha3_256_file(&s[1..(s.len() - 1)]);
            println!("■■ {:02x?}", dig);
        },
        _ => { println!("No command {}", cmd) },
    }
}

