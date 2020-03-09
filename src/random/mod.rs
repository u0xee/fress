// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

pub mod fuzz;

// From George Marsaglia's "Xorshift RNGs", 2003
// https://www.jstatsoft.org/index.php/jss/article/view/v008i14/xorshift.pdf
pub const ABC: [(u8, u8, u8); 275] = [
    ( 1, 1,54), ( 1, 1,55), ( 1, 3,45), ( 1, 7, 9), ( 1, 7,44), ( 1, 7,46), ( 1, 9,50), ( 1,11,35),
    ( 1,11,50), ( 1,13,45), ( 1,15, 4), ( 1,15,63), ( 1,19, 6), ( 1,19,16), ( 1,23,14), ( 1,23,29),
    ( 1,29,34), ( 1,35, 5), ( 1,35,11), ( 1,35,34), ( 1,45,37), ( 1,51,13), ( 1,53, 3), ( 1,59,14),
    ( 2,13,23), ( 2,31,51), ( 2,31,53), ( 2,43,27), ( 2,47,49), ( 3, 1,11), ( 3, 5,21), ( 3,13,59),
    ( 3,21,31), ( 3,25,20), ( 3,25,31), ( 3,25,56), ( 3,29,40), ( 3,29,47), ( 3,29,49), ( 3,35,14),
    ( 3,37,17), ( 3,43, 4), ( 3,43, 6), ( 3,43,11), ( 3,51,16), ( 3,53, 7), ( 3,61,17), ( 3,61,26),
    ( 4, 7,19), ( 4, 9,13), ( 4,15,51), ( 4,15,53), ( 4,29,45), ( 4,29,49), ( 4,31,33), ( 4,35,15),
    ( 4,35,21), ( 4,37,11), ( 4,37,21), ( 4,41,19), ( 4,41,45), ( 4,43,21), ( 4,43,31), ( 4,53, 7),
    ( 5, 9,23), ( 5,11,54), ( 5,15,27), ( 5,17,11), ( 5,23,36), ( 5,33,29), ( 5,41,20), ( 5,45,16),
    ( 5,47,23), ( 5,53,20), ( 5,59,33), ( 5,59,35), ( 5,59,63), ( 6, 1,17), ( 6, 3,49), ( 6,17,47),
    ( 6,23,27), ( 6,27, 7), ( 6,43,21), ( 6,49,29), ( 6,55,17), ( 7, 5,41), ( 7, 5,47), ( 7, 5,55),
    ( 7, 7,20), ( 7, 9,38), ( 7,11,10), ( 7,11,35), ( 7,13,58), ( 7,19,17), ( 7,19,54), ( 7,23, 8),
    ( 7,25,58), ( 7,27,59), ( 7,33, 8), ( 7,41,40), ( 7,43,28), ( 7,51,24), ( 7,57,12), ( 8, 5,59),
    ( 8, 9,25), ( 8,13,25), ( 8,13,61), ( 8,15,21), ( 8,25,59), ( 8,29,19), ( 8,31,17), ( 8,37,21),
    ( 8,51,21), ( 9, 1,27), ( 9, 5,36), ( 9, 5,43), ( 9, 7,18), ( 9,19,18), ( 9,21,11), ( 9,21,20),
    ( 9,21,40), ( 9,23,57), ( 9,27,10), ( 9,29,12), ( 9,29,37), ( 9,37,31), ( 9,41,45), (10, 7,33),
    (10,27,59), (10,53,13), (11, 5,32), (11, 5,34), (11, 5,43), (11, 5,45), (11, 9,14), (11, 9,34),
    (11,13,40), (11,15,37), (11,23,42), (11,23,56), (11,25,48), (11,27,26), (11,29,14), (11,31,18),
    (11,53,23), (12, 1,31), (12, 3,13), (12, 3,49), (12, 7,13), (12,11,47), (12,25,27), (12,39,49),
    (12,43,19), (13, 3,40), (13, 3,53), (13, 7,17), (13, 9,15), (13, 9,50), (13,13,19), (13,17,43),
    (13,19,28), (13,19,47), (13,21,18), (13,21,49), (13,29,35), (13,35,30), (13,35,38), (13,47,23),
    (13,51,21), (14,13,17), (14,15,19), (14,23,33), (14,31,45), (14,47,15), (15, 1,19), (15, 5,37),
    (15,13,28), (15,13,52), (15,17,27), (15,19,63), (15,21,46), (15,23,23), (15,45,17), (15,47,16),
    (15,49,26), (16, 5,17), (16, 7,39), (16,11,19), (16,11,27), (16,13,55), (16,21,35), (16,25,43),
    (16,27,53), (16,47,17), (17,15,58), (17,23,29), (17,23,51), (17,23,52), (17,27,22), (17,45,22),
    (17,47,28), (17,47,29), (17,47,54), (18, 1,25), (18, 3,43), (18,19,19), (18,25,21), (18,41,23),
    (19, 7,36), (19, 7,55), (19,13,37), (19,15,46), (19,21,52), (19,25,20), (19,41,21), (19,43,27),
    (20, 1,31), (20, 5,29), (21, 1,27), (21, 9,29), (21,13,52), (21,15,28), (21,15,29), (21,17,24),
    (21,17,30), (21,17,48), (21,21,32), (21,21,34), (21,21,37), (21,21,38), (21,21,40), (21,21,41),
    (21,21,43), (21,41,23), (22, 3,39), (23, 9,38), (23, 9,48), (23, 9,57), (23,13,38), (23,13,58),
    (23,13,61), (23,17,25), (23,17,54), (23,17,56), (23,17,62), (23,41,34), (23,41,51), (24, 9,35),
    (24,11,29), (24,25,25), (24,31,35), (25, 7,46), (25, 7,49), (25, 9,39), (25,11,57), (25,13,29),
    (25,13,39), (25,13,62), (25,15,47), (25,21,44), (25,27,27), (25,27,53), (25,33,36), (25,39,54),
    (28, 9,55), (28,11,53), (29,27,37), (31, 1,51), (31,25,37), (31,27,35), (33,31,43), (33,31,55),
    (43,21,46), (49,15,61), (55, 9,56),
];

pub fn cycle_abc(abc: usize, mut y: u64) -> u64 {
    let (a, b, c) = ABC[abc];
    y ^= y << a as u64;
    y ^= y >> b as u64;
    y ^= y << c as u64;
    y
}

pub fn cycle(y: u64) -> u64 {
    cycle_abc(155, y)
}

pub fn cycle_n(mut y: u64, n: u32) -> u64 {
    for _ in 0..n {
        y = cycle(y);
    }
    y
}

// From Allen B. Downey's "Generating Pseudo-random Floating-Point Values", 2007
// http://allendowney.com/research/rand/downey07randfloat.pdf
pub fn uniform_f64(exp_seed: u64, mantissa_seed: u64) -> f64 {
    let o = exponent_offset(exp_seed) as u64;
    let exp = 0x3FEu64 /*exponent for 1/2*/ - o;
    let mantissa_mask = (1u64 << 52) - 1;
    let bits = (mantissa_seed & mantissa_mask) | (exp << 52);
    f64::from_bits(bits)
}

fn exponent_offset(y: u64) -> u32 {
    let mask = (1u64 << 8) - 1;
    let mut coin_flips = 0u64;
    for i in 0..8 {
        let ones_in_byte = (y & (mask << (i << 3))).count_ones();
        coin_flips |= (ones_in_byte as u64 & 1) << i;
    }
    let top_bits = (y ^ (y >> 32)) | (1 << 32);
    coin_flips |= top_bits << 8;
    coin_flips.trailing_zeros()
}

// From George Marsaglia's "A Convenient Method for Generating Normal Variables", 1964
// https://epubs.siam.org/doi/abs/10.1137/1006063
// case_4 from http://www.wolframalpha.com/input/?i=N%5B2*(2*pi)%5E(-1%2F2)+*+(integrate+e%5E(-0.5+*+x%5E2)+from+3+to+infinity),+20%5D
pub fn normal_f64(seedling: u64) -> f64 {
    let case_1 = 0.8638;
    let case_2 =  0.1107;
    let case_4 = 0.00269979606326019;

    let seed = cycle(seedling.swap_bytes());
    let p = uniform_f64(seed, cycle(seed));
    let seed = cycle(cycle(seed));
    if p > (1f64 - case_1) {
        let u1 = uniform_f64(seed, cycle(seed));
        let seed2 = cycle(cycle(seed));
        let u2 = uniform_f64(seed2, cycle(seed2));
        let seed3 = cycle(cycle(seed2));
        let u3 = uniform_f64(seed3, cycle(seed3));
        2.0 * (u1 + u2 + u3 - 1.5)
    } else if p > (1f64 - case_1 - case_2) {
        let u1 = uniform_f64(seed, cycle(seed));
        let seed2 = cycle(cycle(seed));
        let u2 = uniform_f64(seed2, cycle(seed2));
        1.5 * (u1 + u2 - 1.0)
    } else if p > case_4 {
        let mut seed = seed;
        for _ in 0..100 {
            let u1 = uniform_f64(seed, cycle(seed));
            let seed2 = cycle(cycle(seed));
            let u2 = uniform_f64(seed2, cycle(seed2));
            let x = 6.0 * u1 - 3.0;
            let y = 0.358 * u2;
            if y < g3(x) {
                return x;
            } else {
                seed = cycle(cycle(seed2));
            }
        }
        panic!("normal_f64 case_3 drew unsuccessfully 100 times");
    } else {
        let mut seed = seed;
        for _ in 0..100 {
            let u1 = uniform_f64(seed, cycle(seed));
            let seed2 = cycle(cycle(seed));
            let u2 = uniform_f64(seed2, cycle(seed2));
            let seed3 = cycle(cycle(seed2));
            let mask = (1u64 << 32) - 1;
            let v1 = u1 * (-1f64).powi(((seed3 & mask).count_ones() & 1) as i32);
            let v2 = u2 * (-1f64).powi((((seed3 >> 32) & mask).count_ones() & 1) as i32);
            let r = v1.powi(2) + v2.powi(2);
            if r < 1.0 {
                let w = -(r.ln());
                let common = ((9.0 + 2.0 * w) / r).sqrt();
                let x = v1 * common;
                let y = v2 * common;
                if x.abs() > 3.0 {
                    return x;
                }
                if y.abs() > 3.0 {
                    return y;
                }
            }
            seed = cycle(seed3);
        }
        panic!("normal_f64 case_4 drew unsuccessfully 100 times");
    }
}

fn g3(x: f64) -> f64 {
    let abs_x = x.abs();
    let x2 = x.powi(2);
    let common = 17.49731196 * (-0.5 * x2).exp();
    let common1 = 2.15787544 * (1.5 - abs_x);
    let common2 = 2.36785163 * (3.0 - abs_x).powi(2);
    if abs_x < 1.0 {
        common - 4.73570326 * (3.0 - x2) - common1
    } else if abs_x < 1.5 {
        common - common1 - common2
    } else {
        common - common2
    }
}


pub const PI: [u64; 512] = [
    0x243f6a88_85a308d3, 0x13198a2e_03707344, 0xa4093822_299f31d0, 0x082efa98_ec4e6c89,
    0x452821e6_38d01377, 0xbe5466cf_34e90c6c, 0xc0ac29b7_c97c50dd, 0x3f84d5b5_b5470917,
    0x9216d5d9_8979fb1b, 0xd1310ba6_98dfb5ac, 0x2ffd72db_d01adfb7, 0xb8e1afed_6a267e96,
    0xba7c9045_f12c7f99, 0x24a19947_b3916cf7, 0x0801f2e2_858efc16, 0x636920d8_71574e69,
    0xa458fea3_f4933d7e, 0x0d95748f_728eb658, 0x718bcd58_82154aee, 0x7b54a41d_c25a59b5,
    0x9c30d539_2af26013, 0xc5d1b023_286085f0, 0xca417918_b8db38ef, 0x8e79dcb0_603a180e,
    0x6c9e0e8b_b01e8a3e, 0xd71577c1_bd314b27, 0x78af2fda_55605c60, 0xe65525f3_aa55ab94,
    0x57489862_63e81440, 0x55ca396a_2aab10b6, 0xb4cc5c34_1141e8ce, 0xa15486af_7c72e993,
    0xb3ee1411_636fbc2a, 0x2ba9c55d_741831f6, 0xce5c3e16_9b87931e, 0xafd6ba33_6c24cf5c,
    0x7a325381_28958677, 0x3b8f4898_6b4bb9af, 0xc4bfe81b_66282193, 0x61d809cc_fb21a991,
    0x487cac60_5dec8032, 0xef845d5d_e98575b1, 0xdc262302_eb651b88, 0x23893e81_d396acc5,
    0x0f6d6ff3_83f44239, 0x2e0b4482_a4842004, 0x69c8f04a_9e1f9b5e, 0x21c66842_f6e96c9a,
    0x670c9c61_abd388f0, 0x6a51a0d2_d8542f68, 0x960fa728_ab5133a3, 0x6eef0b6c_137a3be4,
    0xba3bf050_7efb2a98, 0xa1f1651d_39af0176, 0x66ca593e_82430e88, 0x8cee8619_456f9fb4,
    0x7d84a5c3_3b8b5ebe, 0xe06f75d8_85c12073, 0x401a449f_56c16aa6, 0x4ed3aa62_363f7706,
    0x1bfedf72_429b023d, 0x37d0d724_d00a1248, 0xdb0fead3_49f1c09b, 0x075372c9_80991b7b,
    0x25d479d8_f6e8def7, 0xe3fe501a_b6794c3b, 0x976ce0bd_04c006ba, 0xc1a94fb6_409f60c4,
    0x5e5c9ec2_196a2463, 0x68fb6faf_3e6c53b5, 0x1339b2eb_3b52ec6f, 0x6dfc511f_9b30952c,
    0xcc814544_af5ebd09, 0xbee3d004_de334afd, 0x660f2807_192e4bb3, 0xc0cba857_45c8740f,
    0xd20b5f39_b9d3fbdb, 0x5579c0bd_1a60320a, 0xd6a100c6_402c7279, 0x679f25fe_fb1fa3cc,
    0x8ea5e9f8_db3222f8, 0x3c7516df_fd616b15, 0x2f501ec8_ad0552ab, 0x323db5fa_fd238760,
    0x53317b48_3e00df82, 0x9e5c57bb_ca6f8ca0, 0x1a87562e_df1769db, 0xd542a8f6_287effc3,
    0xac6732c6_8c4f5573, 0x695b27b0_bbca58c8, 0xe1ffa35d_b8f011a0, 0x10fa3d98_fd2183b8,
    0x4afcb56c_2dd1d35b, 0x9a53e479_b6f84565, 0xd28e49bc_4bfb9790, 0xe1ddf2da_a4cb7e33,
    0x62fb1341_cee4c6e8, 0xef20cada_36774c01, 0xd07e9efe_2bf11fb4, 0x95dbda4d_ae909198,
    0xeaad8e71_6b93d5a0, 0xd08ed1d0_afc725e0, 0x8e3c5b2f_8e7594b7, 0x8ff6e2fb_f2122b64,
    0x8888b812_900df01c, 0x4fad5ea0_688fc31c, 0xd1cff191_b3a8c1ad, 0x2f2f2218_be0e1777,
    0xea752dfe_8b021fa1, 0xe5a0cc0f_b56f74e8, 0x18acf3d6_ce89e299, 0xb4a84fe0_fd13e0b7,
    0x7cc43b81_d2ada8d9, 0x165fa266_80957705, 0x93cc7314_211a1477, 0xe6ad2065_77b5fa86,
    0xc75442f5_fb9d35cf, 0xebcdaf0c_7b3e89a0, 0xd6411bd3_ae1e7e49, 0x00250e2d_2071b35e,
    0x226800bb_57b8e0af, 0x2464369b_f009b91e, 0x5563911d_59dfa6aa, 0x78c14389_d95a537f,
    0x207d5ba2_02e5b9c5, 0x83260376_6295cfa9, 0x11c81968_4e734a41, 0xb3472dca_7b14a94a,
    0x1b510052_9a532915, 0xd60f573f_bc9bc6e4, 0x2b60a476_81e67400, 0x08ba6fb5_571be91f,
    0xf296ec6b_2a0dd915, 0xb6636521_e7b9f9b6, 0xff34052e_c5855664, 0x53b02d5d_a99f8fa1,
    0x08ba4799_6e85076a, 0x4b7a70e9_b5b32944, 0xdb75092e_c4192623, 0xad6ea6b0_49a7df7d,
    0x9cee60b8_8fedb266, 0xecaa8c71_699a17ff, 0x5664526c_c2b19ee1, 0x193602a5_75094c29,
    0xa0591340_e4183a3e, 0x3f54989a_5b429d65, 0x6b8fe4d6_99f73fd6, 0xa1d29c07_efe830f5,
    0x4d2d38e6_f0255dc1, 0x4cdd2086_8470eb26, 0x6382e9c6_021ecc5e, 0x09686b3f_3ebaefc9,
    0x3c971814_6b6a70a1, 0x687f3584_52a0e286, 0xb79c5305_aa500737, 0x3e07841c_7fdeae5c,
    0x8e7d44ec_5716f2b8, 0xb03ada37_f0500c0d, 0xf01c1f04_0200b3ff, 0xae0cf51a_3cb574b2,
    0x25837a58_dc0921bd, 0xd19113f9_7ca92ff6, 0x94324773_22f54701, 0x3ae5e581_37c2dadc,
    0xc8b57634_9af3dda7, 0xa9446146_0fd0030e, 0xecc8c73e_a4751e41, 0xe238cd99_3bea0e2f,
    0x3280bba1_183eb331, 0x4e548b38_4f6db908, 0x6f420d03_f60a04bf, 0x2cb81290_24977c79,
    0x5679b072_bcaf89af, 0xde9a771f_d9930810, 0xb38bae12_dccf3f2e, 0x5512721f_2e6b7124,
    0x501adde6_9f84cd87, 0x7a584718_7408da17, 0xbc9f9abc_e94b7d8c, 0xec7aec3a_db851dfa,
    0x63094366_c464c3d2, 0xef1c1847_3215d908, 0xdd433b37_24c2ba16, 0x12a14d43_2a65c451,
    0x50940002_133ae4dd, 0x71dff89e_10314e55, 0x81ac77d6_5f11199b, 0x043556f1_d7a3c76b,
    0x3c11183b_5924a509, 0xf28fe6ed_97f1fbfa, 0x9ebabf2c_1e153c6e, 0x86e34570_eae96fb1,
    0x860e5e0a_5a3e2ab3, 0x771fe71c_4e3d06fa, 0x2965dcb9_99e71d0f, 0x803e89d6_5266c825,
    0x2e4cc978_9c10b36a, 0xc6150eba_94e2ea78, 0xa5fc3c53_1e0a2df4, 0xf2f74ea7_361d2b3d,
    0x1939260f_19c27960, 0x5223a708_f71312b6, 0xebadfe6e_eac31f66, 0xe3bc4595_a67bc883,
    0xb17f37d1_018cff28, 0xc332ddef_be6c5aa5, 0x65582185_68ab9802, 0xeecea50f_db2f953b,
    0x2aef7dad_5b6e2f84, 0x1521b628_29076170, 0xecdd4775_619f1510, 0x13cca830_eb61bd96,
    0x0334fe1e_aa0363cf, 0xb5735c90_4c70a239, 0xd59e9e0b_cbaade14, 0xeecc86bc_60622ca7,
    0x9cab5cab_b2f3846e, 0x648b1eaf_19bdf0ca, 0xa02369b9_655abb50, 0x40685a32_3c2ab4b3,
    0x319ee9d5_c021b8f7, 0x9b540b19_875fa099, 0x95f7997e_623d7da8, 0xf837889a_97e32d77,
    0x11ed935f_16681281, 0x0e358829_c7e61fd6, 0x96dedfa1_7858ba99, 0x57f584a5_1b227263,
    0x9b83c3ff_1ac24696, 0xcdb30aeb_532e3054, 0x8fd948e4_6dbc3128, 0x58ebf2ef_34c6ffea,
    0xfe28ed61_ee7c3c73, 0x5d4a14d9_e864b7e3, 0x42105d14_203e13e0, 0x45eee2b6_a3aaabea,
    0xdb6c4f15_facb4fd0, 0xc742f442_ef6abbb5, 0x654f3b1d_41cd2105, 0xd81e799e_86854dc7,
    0xe44b476a_3d816250, 0xcf62a1f2_5b8d2646, 0xfc8883a0_c1c7b6a3, 0x7f1524c3_69cb7492,
    0x47848a0b_5692b285, 0x095bbf00_ad19489d, 0x1462b174_23820e00, 0x58428d2a_0c55f5ea,
    0x1dadf43e_233f7061, 0x3372f092_8d937e41, 0xd65fecf1_6c223bdb, 0x7cde3759_cbee7460,
    0x4085f2a7_ce77326e, 0xa6078084_19f8509e, 0xe8efd855_61d99735, 0xa969a7aa_c50c06c2,
    0x5a04abfc_800bcadc, 0x9e447a2e_c3453484, 0xfdd56705_0e1e9ec9, 0xdb73dbd3_105588cd,
    0x675fda79_e3674340, 0xc5c43465_713e38d8, 0x3d28f89e_f16dff20, 0x153e21e7_8fb03d4a,
    0xe6e39f2b_db83adf7, 0xe93d5a68_948140f7, 0xf64c261c_94692934, 0x411520f7_7602d4f7,
    0xbcf46b2e_d4a20068, 0xd4082471_3320f46a, 0x43b7d4b7_500061af, 0x1e39f62e_97244546,
    0x14214f74_bf8b8840, 0x4d95fc1d_96b591af, 0x70f4ddd3_66a02f45, 0xbfbc09ec_03bd9785,
    0x7fac6dd0_31cb8504, 0x96eb27b3_55fd3941, 0xda2547e6_abca0a9a, 0x28507825_530429f4,
    0x0a2c86da_e9b66dfb, 0x68dc1462_d7486900, 0x680ec0a4_27a18dee, 0x4f3ffea2_e887ad8c,
    0xb58ce006_7af4d6b6, 0xaace1e7c_d3375fec, 0xce78a399_406b2a42, 0x20fe9e35_d9f385b9,
    0xee39d7ab_3b124e8b, 0x1dc9faf7_4b6d1856, 0x26a36631_eae397b2, 0x3a6efa74_dd5b4332,
    0x6841e7f7_ca7820fb, 0xfb0af54e_d8feb397, 0x454056ac_ba489527, 0x55533a3a_20838d87,
    0xfe6ba9b7_d096954b, 0x55a867bc_a1159a58, 0xcca92963_99e1db33, 0xa62a4a56_3f3125f9,
    0x5ef47e1c_9029317c, 0xfdf8e802_04272f70, 0x80bb155c_05282ce3, 0x95c11548_e4c66d22,
    0x48c1133f_c70f86dc, 0x07f9c9ee_41041f0f, 0x404779a4_5d886e17, 0x325f51eb_d59bc0d1,
    0xf2bcc18f_41113564, 0x257b7834_602a9c60, 0xdff8e8a3_1f636c1b, 0x0e12b4c2_02e1329e,
    0xaf664fd1_cad18115, 0x6b2395e0_333e92e1, 0x3b240b62_eebeb922, 0x85b2a20e_e6ba0d99,
    0xde720c8c_2da2f728, 0xd0127845_95b794fd, 0x647d0862_e7ccf5f0, 0x5449a36f_877d48fa,
    0xc39dfd27_f33e8d1e, 0x0a476341_992eff74, 0x3a6f6eab_f4f8fd37, 0xa812dc60_a1ebddf8,
    0x991be14c_db6e6b0d, 0xc67b5510_6d672c37, 0x2765d43b_dcd0e804, 0xf1290dc7_cc00ffa3,
    0xb5390f92_690fed0b, 0x667b9ffb_cedb7d9c, 0xa091cf0b_d9155ea3, 0xbb132f88_515bad24,
    0x7b9479bf_763bd6eb, 0x37392eb3_cc115979, 0x8026e297_f42e312d, 0x6842ada7_c66a2b3b,
    0x12754ccc_782ef11c, 0x6a124237_b79251e7, 0x06a1bbe6_4bfb6350, 0x1a6b1018_11caedfa,
    0x3d25bdd8_e2e1c3c9, 0x44421659_0a121386, 0xd90cec6e_d5abea2a, 0x64af674e_da86a85f,
    0xbebfe988_64e4c3fe, 0x9dbc8057_f0f7c086, 0x60787bf8_6003604d, 0xd1fd8346_f6381fb0,
    0x7745ae04_d736fccc, 0x83426b33_f01eab71, 0xb0804187_3c005e5f, 0x77a057be_bde8ae24,
    0x55464299_bf582e61, 0x4e58f48f_f2ddfda2, 0xf474ef38_8789bdc2, 0x5366f9c3_c8b38e74,
    0xb475f255_46fcd9b9, 0x7aeb2661_8b1ddf84, 0x846a0e79_915f95e2, 0x466e598e_20b45770,
    0x8cd55591_c902de4c, 0xb90bace1_bb8205d0, 0x11a86248_7574a99e, 0xb77f19b6_e0a9dc09,
    0x662d09a1_c4324633, 0xe85a1f02_09f0be8c, 0x4a99a025_1d6efe10, 0x1ab93d1d_0ba5a4df,
    0xa186f20f_2868f169, 0xdcb7da83_573906fe, 0xa1e2ce9b_4fcd7f52, 0x50115e01_a70683fa,
    0xa002b5c4_0de6d027, 0x9af88c27_773f8641, 0xc3604c06_61a806b5, 0xf0177a28_c0f586e0,
    0x006058aa_30dc7d62, 0x11e69ed7_2338ea63, 0x53c2dd94_c2c21634, 0xbbcbee56_90bcb6de,
    0xebfc7da1_ce591d76, 0x6f05e409_4b7c0188, 0x39720a3d_7c927c24, 0x86e3725f_724d9db9,
    0x1ac15bb4_d39eb8fc, 0xed545578_08fca5b5, 0xd83d7cd3_4dad0fc4, 0x1e50ef5e_b161e6f8,
    0xa28514d9_6c51133c, 0x6fd5c7e7_56e14ec4, 0x362abfce_ddc6c837, 0xd79a3234_92638212,
    0x670efa8e_406000e0, 0x3a39ce37_d3faf5cf, 0xabc27737_5ac52d1b, 0x5cb0679e_4fa33742,
    0xd3822740_99bc9bbe, 0xd5118e9d_bf0f7315, 0xd62d1c7e_c700c47b, 0xb78c1b6b_21a19045,
    0xb26eb1be_6a366eb4, 0x5748ab2f_bc946e79, 0xc6a376d2_6549c2c8, 0x530ff8ee_468dde7d,
    0xd5730a1d_4cd04dc6, 0x2939bbdb_a9ba4650, 0xac9526e8_be5ee304, 0xa1fad5f0_6a2d519a,
    0x63ef8ce2_9a86ee22, 0xc089c2b8_43242ef6, 0xa51e03aa_9cf2d0a4, 0x83c061ba_9be96a4d,
    0x8fe51550_ba645bd6, 0x2826a2f9_a73a3ae1, 0x4ba99586_ef5562e9, 0xc72fefd3_f752f7da,
    0x3f046f69_77fa0a59, 0x80e4a915_87b08601, 0x9b09e6ad_3b3ee593, 0xe990fd5a_9e34d797,
    0x2cf0b7d9_022b8b51, 0x96d5ac3a_017da67d, 0xd1cf3ed6_7c7d2d28, 0x1f9f25cf_adf2b89b,
    0x5ad6b472_5a88f54c, 0xe029ac71_e019a5e6, 0x47b0acfd_ed93fa9b, 0xe8d3c48d_283b57cc,
    0xf8d56629_79132e28, 0x785f0191_ed756055, 0xf7960e44_e3d35e8c, 0x15056dd4_88f46dba,
    0x03a16125_0564f0bd, 0xc3eb9e15_3c9057a2, 0x97271aec_a93a072a, 0x1b3f6d9b_1e6321f5,
    0xf59c66fb_26dcf319, 0x7533d928_b155fdf5, 0x03563482_8aba3cbb, 0x28517711_c20ad9f8,
    0xabcc5167_ccad925f, 0x4de81751_3830dc8e, 0x379d5862_9320f991, 0xea7a90c2_fb3e7bce,
    0x5121ce64_774fbe32, 0xa8b6e37e_c3293d46, 0x48de5369_6413e680, 0xa2ae0810_dd6db224,
    0x69852dfd_09072166, 0xb39a460a_6445c0dd, 0x586cdecf_1c20c8ae, 0x5bbef7dd_1b588d40,
    0xccd2017f_6bb4e3bb, 0xdda26a7e_3a59ff45, 0x3e350a44_bcb4cdd5, 0x72eacea8_fa6484bb,
    0x8d6612ae_bf3c6f47, 0xd29be463_542f5d9e, 0xaec2771b_f64e6370, 0x740e0d8d_e75b1357,
    0xf8721671_af537d5d, 0x4040cb08_4eb4e2cc, 0x34d2466a_0115af84, 0xe1b00428_95983a1d,
    0x06b89fb4_ce6ea048, 0x6f3f3b82_3520ab82, 0x011a1d4b_277227f8, 0x611560b1_e7933fdc,
    0xbb3a792b_344525bd, 0xa08839e1_51ce794b, 0x2f32c9b7_a01fbac9, 0xe01cc87e_bcc7d1f6,
    0xcf0111c3_a1e8aac7, 0x1a908749_d44fbd9a, 0xd0dadecb_d50ada38, 0x0339c32a_c6913667,
    0x8df9317c_e0b12b4f, 0xf79e59b7_43f5bb3a, 0xf2d519ff_27d9459c, 0xbf97222c_15e6fc2a,
    0x0f91fc71_9b941525, 0xfae59361_ceb69ceb, 0xc2a86459_12baa8d1, 0xb6c1075e_e3056a0c,
    0x10d25065_cb03a442, 0xe0ec6e0e_1698db3b, 0x4c98a0be_3278e964, 0x9f1f9532_e0d392df,
    0xd3a0342b_8971f21e, 0x1b0a7441_4ba3348c, 0xc5be7120_c37632d8, 0xdf359f8d_9b992f2e,
    0xe60b6f47_0fe3f11d, 0xe54cda54_1edad891, 0xce6279cf_cd3e7e6f, 0x1618b166_fd2c1d05,
    0x848fd2c5_f6fb2299, 0xf523f357_a6327623, 0x93a83531_56cccd02, 0xacf08162_5a75ebb5,
    0x6e163697_88d273cc, 0xde966292_81b949d0, 0x4c50901b_71c65614, 0xe6c6c7bd_327a140a,
    0x45e1d006_c3f27b9a, 0xc9aa53fd_62a80f00, 0xbb25bfe2_35bdd2f6, 0x71126905_b2040222,
    0xb6cbcf7c_cd769c2b, 0x53113ec0_1640e3d3, 0x38abbd60_2547adf0, 0xba38209c_f746ce76,
];


// rand int
