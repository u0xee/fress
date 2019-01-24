// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


// sponge construction
// f permutation, padding rule, r bitrate, message, d bits output
// pad, absorb, squeeze

// sha3-256(M) = keccak[512](M||01, 256)

// different bitrates, capacities
// padding for messages, multi-rate padding


// endpoint for keccak-p[rounds], sha3-256(M), k12(M,C,d), kravatte-sane
// keccak-f[1600] = keccak-p[1600, 24]
// keccak[c] = sponge[keccak-f[1600], pad10*1, 1600 - c]



pub fn hash_zero_message() {
    let mut a = [0u64; 25];
    a[0] = 0x06;
    a[16] = 0x80000000_00000000;
    print_bytes(&a);
    permute(&mut a, 24);
    print_bytes(&a);
}

pub fn print_bytes(a: &[u64]) {
    println!("State in bytes");
    for i in 0..25 {
        for b in 0..8 {
            let x = (a[i] >> (b << 3)) as u8;
            print!("{:02X} ", x);
        }
        if i % 2 == 1 {
            println!();
        }
    }
    println!();
}

// keccak-p[1600, rounds]
pub fn permute(a: &mut [u64], rounds: u32) {
    assert!(rounds <= 24 && rounds == (rounds >> 1) << 1);
    assert_eq!(a.len(), 25);
    let mut e = [0u64; 25];
    let mut c = [0u64; 5];
    sum_columns(a, &mut c);

    let first_idx = 24 - rounds;
    for pair in 0..(rounds >> 1) {
        let idx = (first_idx + (pair << 1)) as usize;
        {
            round(a, &mut e, &mut c);
            e[0] ^= RC[idx]; c[0] ^= RC[idx];
        }
        {
            round(&e, a, &mut c);
            a[0] ^= RC[idx + 1]; c[0] ^= RC[idx + 1];
        }
    }
}

const RC: [u64; 24] = [
    0x00000000_00000001, 0x00000000_00008082, 0x80000000_0000808A, 0x80000000_80008000,
    0x00000000_0000808B, 0x00000000_80000001, 0x80000000_80008081, 0x80000000_00008009,
    0x00000000_0000008A, 0x00000000_00000088, 0x00000000_80008009, 0x00000000_8000000A,
    0x00000000_8000808B, 0x80000000_0000008B, 0x80000000_00008089, 0x80000000_00008003,
    0x80000000_00008002, 0x80000000_00000080, 0x00000000_0000800A, 0x80000000_8000000A,
    0x80000000_80008081, 0x80000000_00008080, 0x00000000_80000001, 0x80000000_80008008,
];

const RHO: [u32; 25] = [
    0 , 1 , 62, 28, 27,
    36, 44, 6 , 55, 20,
    3 , 10, 43, 25, 39,
    41, 45, 15, 21, 8 ,
    18, 2 , 61, 56, 14,
];

const PI: [usize; 25] = [
    0 + 0 * 5, 1 + 1 * 5, 2 + 2 * 5, 3 + 3 * 5, 4 + 4 * 5,
    3 + 0 * 5, 4 + 1 * 5, 0 + 2 * 5, 1 + 3 * 5, 2 + 4 * 5,
    1 + 0 * 5, 2 + 1 * 5, 3 + 2 * 5, 4 + 3 * 5, 0 + 4 * 5,
    4 + 0 * 5, 0 + 1 * 5, 1 + 2 * 5, 2 + 3 * 5, 3 + 4 * 5,
    2 + 0 * 5, 3 + 1 * 5, 4 + 2 * 5, 0 + 3 * 5, 1 + 4 * 5,
];

fn sum_columns(a: &[u64], c: &mut [u64]) {
    c[0] = a[0] ^ a[5] ^ a[10] ^ a[15] ^ a[20];
    c[1] = a[1] ^ a[6] ^ a[11] ^ a[16] ^ a[21];
    c[2] = a[2] ^ a[7] ^ a[12] ^ a[17] ^ a[22];
    c[3] = a[3] ^ a[8] ^ a[13] ^ a[18] ^ a[23];
    c[4] = a[4] ^ a[9] ^ a[14] ^ a[19] ^ a[24];
}

fn round(a: &[u64], e: &mut [u64], c: &mut [u64]) {
    let d0 = c[4] ^ c[1].rotate_left(1);
    let d1 = c[0] ^ c[2].rotate_left(1);
    let d2 = c[1] ^ c[3].rotate_left(1);
    let d3 = c[2] ^ c[4].rotate_left(1);
    let d4 = c[3] ^ c[0].rotate_left(1);

    let mut c0 = 0u64;
    let mut c1 = 0u64;
    let mut c2 = 0u64;
    let mut c3 = 0u64;
    let mut c4 = 0u64;

    {
        let b0 = (a[PI[0]] ^ d0).rotate_left(RHO[PI[0]]);
        let b1 = (a[PI[1]] ^ d1).rotate_left(RHO[PI[1]]);
        let b2 = (a[PI[2]] ^ d2).rotate_left(RHO[PI[2]]);
        let b3 = (a[PI[3]] ^ d3).rotate_left(RHO[PI[3]]);
        let b4 = (a[PI[4]] ^ d4).rotate_left(RHO[PI[4]]);

        let e0 = b0 ^ (!b1 & b2); e[0] = e0; c0 ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[1] = e1; c1 ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[2] = e2; c2 ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[3] = e3; c3 ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[4] = e4; c4 ^= e4;
    }

    {
        let b0 = (a[PI[5]] ^ d3).rotate_left(RHO[PI[5]]);
        let b1 = (a[PI[6]] ^ d4).rotate_left(RHO[PI[6]]);
        let b2 = (a[PI[7]] ^ d0).rotate_left(RHO[PI[7]]);
        let b3 = (a[PI[8]] ^ d1).rotate_left(RHO[PI[8]]);
        let b4 = (a[PI[9]] ^ d2).rotate_left(RHO[PI[9]]);

        let e0 = b0 ^ (!b1 & b2); e[5] = e0; c0 ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[6] = e1; c1 ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[7] = e2; c2 ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[8] = e3; c3 ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[9] = e4; c4 ^= e4;
    }

    {
        let b0 = (a[PI[10]] ^ d1).rotate_left(RHO[PI[10]]);
        let b1 = (a[PI[11]] ^ d2).rotate_left(RHO[PI[11]]);
        let b2 = (a[PI[12]] ^ d3).rotate_left(RHO[PI[12]]);
        let b3 = (a[PI[13]] ^ d4).rotate_left(RHO[PI[13]]);
        let b4 = (a[PI[14]] ^ d0).rotate_left(RHO[PI[14]]);

        let e0 = b0 ^ (!b1 & b2); e[10] = e0; c0 ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[11] = e1; c1 ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[12] = e2; c2 ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[13] = e3; c3 ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[14] = e4; c4 ^= e4;
    }

    {
        let b0 = (a[PI[15]] ^ d4).rotate_left(RHO[PI[15]]);
        let b1 = (a[PI[16]] ^ d0).rotate_left(RHO[PI[16]]);
        let b2 = (a[PI[17]] ^ d1).rotate_left(RHO[PI[17]]);
        let b3 = (a[PI[18]] ^ d2).rotate_left(RHO[PI[18]]);
        let b4 = (a[PI[19]] ^ d3).rotate_left(RHO[PI[19]]);

        let e0 = b0 ^ (!b1 & b2); e[15] = e0; c0 ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[16] = e1; c1 ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[17] = e2; c2 ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[18] = e3; c3 ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[19] = e4; c4 ^= e4;
    }

    {
        let b0 = (a[PI[20]] ^ d2).rotate_left(RHO[PI[20]]);
        let b1 = (a[PI[21]] ^ d3).rotate_left(RHO[PI[21]]);
        let b2 = (a[PI[22]] ^ d4).rotate_left(RHO[PI[22]]);
        let b3 = (a[PI[23]] ^ d0).rotate_left(RHO[PI[23]]);
        let b4 = (a[PI[24]] ^ d1).rotate_left(RHO[PI[24]]);

        let e0 = b0 ^ (!b1 & b2); e[20] = e0; c0 ^= e0;
        let e1 = b1 ^ (!b2 & b3); e[21] = e1; c1 ^= e1;
        let e2 = b2 ^ (!b3 & b4); e[22] = e2; c2 ^= e2;
        let e3 = b3 ^ (!b4 & b0); e[23] = e3; c3 ^= e3;
        let e4 = b4 ^ (!b0 & b1); e[24] = e4; c4 ^= e4;
    }

    c[0] = c0;
    c[1] = c1;
    c[2] = c2;
    c[3] = c3;
    c[4] = c4;
}

