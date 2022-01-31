// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;

fn parse_diagnostics(s: String) {
    use fress::Value;
    use fress::handle::Handle;
    use fress::edn;
    use fress::edn::reader::{EdnReader, ReadResult};

    let mut reader = EdnReader::new();
    // loop while bytes left, reading and reporting on errors or successes
    let r = edn::read(&mut reader, s.as_bytes());
    match r {
        ReadResult::Ok { bytes_used, value } => {
            println!("bytes_used: {}, {}", bytes_used, value.handle());
            value.handle().retire();
        },
        _ => {
            println!("{:?}", r);
        },
    }
    let x: Value = 45.into();
    let y: Value = 43.into();
    let w = fress::read(":name");
    let z = fress::read("{:name 4 :drive 5}");
    println!("XXX {}", z.contains(&w));
    println!("XXX {}", x < y);
}

fn main() {
    use fress::memory::segment;
    let (new_a, free_a) = segment::new_free_counts();

    let filename = {
        use std::env;
        env::args().nth(1).expect("Provide a filename as an argument!")
    };
    use std::fs;
    let s = fs::read_to_string(&filename).unwrap();
    println!("@@ {}", &filename);
    println!("   0 2 4 6 8 a c e 6 8 20  24  28  32  36  40  44  48  52  56  60");
    for (line, txt) in s.lines().enumerate() {
        println!("{:2} |{}", line + 1, txt);
    }
    parse_diagnostics(s);

    {
        let (new_b, free_b) = segment::new_free_counts();
        let new_diff = new_b - new_a;
        let free_diff = free_b - free_a;
        println!("New diff: {}, free diff: {}, new - free: {}", new_diff, free_diff, new_diff - free_diff);
    }
}
