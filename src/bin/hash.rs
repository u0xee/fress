// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress_rust;
use fress_rust::hash::*;

fn main() {
    use fress_rust::hash::keccak::hash_zero_message;
    hash_zero_message();

    /*
    let count = 1000;
    for i in 50..55 {
        //let rs = random_states(count, 0x3f84d5b5_b5470917);
        let rs = range_states(count);
        let rsh = hash_each(|x| h(x), &rs);
        let rflips = one_bit_flip(|x| h(x), &rsh,
                                  i);
        let rfreqs = counts_to_freqs(count, &rflips);
        print_8x8(&rfreqs);
    }
    */
    /*
    println!("hash_64(0, 0) => {:8X}", hash_64(0, 0));
    println!("hash_64(7, 1) => {:8X}", hash_64(7, 1));
    println!("hash_64(8, 1) => {:8X}", hash_64(8, 1));

    let mut counts = vec![0; 32];

    for i in 0..10000 {
        let h = hash_64(i, 1);
        //println!("hash_64({}, 1) => {:08X}", i, h);
        counts[h as usize >> 27] += 1;
    }

    for i in 0..32 {
        println!("bucket {:02}: {:4}", i, counts[i]);
    }

    println!("Hi: {}, lo: {}", counts.iter().max().unwrap(), counts.iter().min().unwrap())
*/
    /*
    //println!("Hello, world!");
    let pi_bytes = 0x243A6A;
    let x = 0xFFFFFFu64;

    for i in 0..4 {
        let t = twist_and_turn(x, pi_bytes, i, 15);
        //println!("x: {:016X} -{}-> {:016X}", x, i, t);
    }

    let y  = 0x966E8E408418A81Du64;
    //println!("turbine: {:016X} -> {:016X}", y, turbine(y));
    */
    //let (f, nf) = avalanche(1u64);
    //println!("{:08X} {:08X}", f, nf);

    /*
    let mut buckets: Vec<u32> = vec![0; 256];

    for z in 0..100 {
        let hz = hash_u64(z, 8);
        //println!("hash_u64: {:016X} -> {:08X}", z, hz);
        let bottom = hz & 0xFF;
        let top = hz >> 24 & 0xFF;
        buckets[top as usize] += 1;
    }

    println!("{:?}", buckets);
    println!("{} -> {:016X}", 0x1, cycle(1));
    println!("{} -> {:016X}", 0x1, cycle_n(1, 2));
    println!("{} -> {:016X}", 0x1, cycle_n(1, 3));
    println!("{} -> {:016X}", 0x1, cycle_n(1, 4));
    println!("{} -> {:016X}", 0x100, cycle(0x100));
    println!("{} -> {:016X}", 0x2, cycle(0xFFFEFAFF_FFF4FF7F));
*/
    /*
    let mut best: u32 = 0;
    let mut params = (0, 0, 0);
    let mut checked = 0;
    for t in 1..32 {
        for f in 1..16 {
            for e in 1..8 {
                checked += 1;
                let dist = neighbor_mapping_distance(t, f, e);
                if dist > best {
                    best = dist;
                    params = (t, f, e);
                }
            }
        }
    }
    println!("dist: {}, params: {:?}", best, params);
    println!("checked: {}", checked);
    */

    /*
    let mut v: Vec<u32> = vec!();
    for i in 0..64 {
        v.push(index_mapping(i, 14, 11, 7));
    }
    println!("Mappings: {:?}", v);

    for i in 0..64 {
        let bit = 1u64 << i;
        let scrambled = scramble(bit);
        println!("{:016X}", scrambled);
    }
    println!("{:016X}", scramble(0x00000000_FFFFFFFFu64));
    */
}
