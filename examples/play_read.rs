// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;
use fress::value::Value;
use fress::handle::Handle;
use fress::vector;
use fress::memory::segment;
use fress::transduce::Transducers;

use fress::edn;
use std::str;
use fress::symbol;

/*
fn m() {
    let a = b"hello@ 42";
    let y = edn::isolate_symbolic(a);
    let z = str::from_utf8(y.unwrap()).unwrap();
    println!("y is {:?}", y);
    println!("z is {}", z);

    let sym_ = Symbol::new(b"hello/world_common", 0);
    let sym2 = Symbol::new(b"hello", 0);
    println!("Differ: {:?} {:?}", sym_, sym2);
    println!("Show me: {}", sym_.handle());
    println!("Compare: {:?}", sym_.handle().cmp(sym2.handle()));
    println!("Hash: {:#X}", sym_.handle().hash());
    println!("Hash: {:#X}", sym2.handle().hash());

    sym_.handle().retire();
    sym2.handle().retire();
}
*/
fn n() {
    use std::io::{self, Write};
    use std::panic;
    use fress::edn::reader::{EdnReader, ReadResult};
    use fress::edn;
    loop {
        let r = panic::catch_unwind(|| {
            let mut reader = EdnReader::new();
            let mut input = String::new();
            loop {
                print!("=> ");
                io::stdout().flush().ok().expect("Could not flush stdout.");
                match io::stdin().read_line(&mut input) {
                    Ok(n) => {
                        if n == 0 {
                            println!("End of file reached.");
                            return ();
                        }
                        let r = edn::read(&mut reader, input.as_bytes());
                        match r {
                            ReadResult::Ok { bytes_used, value } => {
                                println!("bytes_used: {}, {}", bytes_used, value.handle());
                                value.handle().retire();
                            },
                            _ => {
                                println!("{:?}", r);
                            },
                        }
                        input.clear();
                    }
                    Err(error) => {
                        println!("error: {}", error);
                        return ();
                    },
                }
            }
        });
        match r {
            Ok(_) => { return; },
            _     => { },
        }
    }

}


fn main() {
    let (new_a, free_a) = segment::new_free_counts();
    //m();
    n();
    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}

