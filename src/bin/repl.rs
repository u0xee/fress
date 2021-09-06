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
    let stop_cmd = fress::read(":repl/quit ").unwrap();
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
        let result = format!("■■ {}", v);
        println!("{}", result);
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
                if n == 0 { return fress::read(":repl/quit ").unwrap() }
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

