use super::gf as GF;
use std;
use std::mem;

// pub fn authenticated_encryption(
//     K: &str,
//     IV: Vec<u8>,
//     P: Vec<u8>,
//     A: Vec<u8>,
// ) -> (Vec<u8>, Vec<u8>) {
//     let n = P.len() / 128;
//     let u = P.len() % 128;

//     let m = A.len() / 128;
//     let v = A.len() % 128;

//     let H = E(K, [0; 16]);

//     let mut C = Vec::with_capacity(P.len());
//     // @TODO implement GHASH to support non 96 bit IVs
//     // let Y0 = if IV.len() == 12 {
//     //     // 96 bits
//     //     IV.extend([0, 0, 0, 1].iter())
//     // } else {
//     //     panic!("{:?}", "no");
//     // }
//     let mut Y0 = IV.clone();
//     Y0.extend([0, 0, 0, 1].iter());
//     for y in Y0.iter_mut() {
//         *y = incr(*y);
//     }
//     ///////////// Apply all up to the janky portion /////////

//     let mut block = [0; 16];
//     for _ in 0..n {
//         block.clone_from_slice(&Y0[n * 16..n * 16 + 16]);
//         block = E(K, block);
//         xor(&mut C, P, block);
//     }
//     // for _ in 0..n {
//     //     block.clone_from_slice(&P[n..n + 16]);
//     //     for (i, b) in E(K, block).iter_mut().enumerate() {
//     //         C[i * n] ^= *b;
//     //     }
//     // }

//     // &C[n..].iter();
//     (C, vec![])
// }

// pub fn authenticated_encryption(
//     K: &str,
//     IV: Vec<u8>,
//     P: Vec<u8>,
//     A: Vec<u8>,
// ) -> (Vec<u8>, Vec<u8>) {
//     let H = E(K, [0; 16]);
//     // let J = if IV.len() == 12 {
//     //             IV.extend([0,0,0,1].iter());
//     //             IV
//     //         } else {
//     //             GHASH()
//     //         }
//     (P, vec![])
// }

// pub fn authenticated_decryption(
//     K: &str,
//     IV: Vec<u8>,
//     C: Vec<u8>,
//     A: Vec<u8>,
//     T: Vec<u8>,
// ) -> Result<Vec<u8>, &'static str> {
//     Ok(C)
// }

// fn E(K: &str, X: [u8; 16]) -> [u8; 16] {
//     X
// }

// fn GHASH(H: [u8; 16], X: Vec<u8>, K: Vec<u8>) {
//     // let h: [u64; 2] = unsafe { mem::transmute(H) };
//     let Y = [0; 16];
//     for i in 1..X.len() {
//         // Y[i] = multiply(Y[i - 1] ^ X[i], X[i]);
//     }
// }

// fn incr(i: u8) -> u8 {
//     i
// }

// const R: [u8; 16] = [225, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// fn multiply(X: [u8; 16], Y: [u8; 16]) -> [u8; 16] {
//     let mut Z = [0; 16];
//     let mut V = X;
//     for byte in 0..16 {
//         for bit in 0..8 {
//             if (Y[byte] >> 7 - bit) & 1 == 0 {
//                 for byte in 0..16 {
//                     Z[byte] ^= V[byte]
//                 }
//             }
//             if V[15] & 1 == 0 {
//                 for byte in 0..16 {
//                     V[byte] <<= 1;
//                 }
//             } else {
//                 for byte in 0..16 {
//                     V[byte] = (V[byte] << 1) ^ R[byte]
//                 }
//             }
//         }
//     }
//     return Z;
// }

// fn multiply(X: [u8; 16], Y: [u8; 16]) {
//     let mut Z = [0; 16];
//     let mut V = Y;
//     for byte in 0..16 {
//         for bit in 0..8 {
//             Z[byte] = if (X[byte] >> 7 - bit) & 1 == 0 {
//                 (Z[byte] ^ !(1 << 6 - bit)) | (((Z[byte] >> 7 - bit) & 1) << 6 - bit)
//             } else {
//                 (Z[byte] ^ !(1 << 6 - bit))
//                     | ((((Z[byte] >> 7 - bit) & 1) ^ ((Y[byte] >> 7 - bit) & 1)) << (6 - bit))
//             };
//             V[byte] = if V[byte] & 1 == 0 { V[byte] } else { V[byte] }
//         }
//     }
// }

// fn xor(C: &mut [u8], P: &[u8], block: [u8; 128]) {
//     for (i, c) in C.iter_mut().enumerate() {
//         c* =
//     }
// }

const R: u128 = 0b11100001 << 120;
fn mult(x: u128, y: u128) -> u128 {
    let mut z: u128 = 0;
    let mut v = x;
    for bit in 0..128 {
        if (y >> (127 - bit)) & 1 == 1 {
            z = add(z, v);
        }
        if v & 1 == 0 {
            v >>= 1;
        } else {
            v = add(v >> 1, R);
        }
    }
    z
}

fn add(x: u128, y: u128) -> u128 {
    x ^ y
}

// fn ghash(x: Vec<u128>, h: u128) -> u128 {
//     let mut y: u128 = 0;
//     for i in 1..x.len() {
//         y = mult(add(y, x[i]), h)
//     }
//     y
// }

fn ghash(h: u128, a: Vec<u128>, c: Vec<u128>) -> u128 {
    let m = a.len();
    let n = c.len();
    let mut x: u128 = 0;
    for i in 0..m {
        x = mult(add(x, a[i]), h);
    }
    println!(
        "AAD got {:?} want {:?}",
        x, 0xbcfb3d1d0e6e3e78ba45403377dba11b as u128
    );
    // This only works because a is all u128,
    // @TODO make SURE that the final block is coming in
    // prepadded with 0s to the right.
    // x = add(x, mult(a[m - 1], h));
    for i in m..m + n {
        x = mult(add(x, c[i - m]), h);
    }
    println!(
        "cipher got {:?}, want {:?}",
        x, 0x336fb643310d7bac2aeaa76247f6036d as u128
    );
    // println!("{:?}", c[c.len() - 1].trailing_zeros());
    // I cannot explain this minus one, I just can't
    let len_a = a.len() as u32 * 128 - (a[a.len() - 1].trailing_zeros() - 1);
    let len_c = c.len() as u32 * 128 - c[c.len() - 1].trailing_zeros();
    let len = (len_a as u128) << 64 | len_c as u128;
    println!(
        "len got {:?} want {:?}",
        len, 0x00000000000000600000000000000130 as u128
    );
    mult(add(x, len), h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_test() {
        // authenticated_encryption();
    }

    #[test]
    fn decryption_test() {
        // authenticated_decryption()
    }

    #[test]
    fn multiply_test() {
        let x: u128 = 5;
        let y: u128 = 7;
        let z: u128 = 13;
        // Commutative
        assert_eq!(mult(x, y), mult(y, x));
        // Distributive
        assert_eq!(mult(add(x, y), z), add(mult(x, z), mult(y, z)));
    }

    #[test]
    fn ghash_test() {
        let c: Vec<u128> = vec![
            0xf24de3a3fb34de6cacba861c9d7e4bca,
            0xbe633bd50d294e6f42a5f47a51c7d19b,
            0x36de3adf883300000000000000000000,
        ];
        let h: u128 = 0xc6a13b37878f5b826f4f8162a1c8d879;
        let a: Vec<u128> = vec![0x8040f17b8041f8d35501a0b200000000];
        let r = ghash(h, a, c);
        println!(
            "final got {:?} want {:?}",
            r, 0x1b964067078c408c4e442a8f015e5264 as u128
        );
        // println!("{:?}", 0x899d7f27beb16a9152cf765ee4390cce as u128);
        // let h: u128 = 0x73A23D80121DE2D5A850253FCF43120E;
        // let c: u128 = 0x701AFA1CC039C0D765128A665DAB69243899BF7318CCDC81C9931DA17FBE8EDD7D17CB8B4C26FC81E3284F2B7FBA713D;
    }
}
