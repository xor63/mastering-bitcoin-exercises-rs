//! Solution for Exercise 4.2

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

fn main() {
    let p = BigInt::parse_bytes(b"115792089237316195423570985008\
                                  687907853269984665640564039457\
                                  584007908834671663", 10).unwrap();
    let x = BigInt::parse_bytes(b"550662630222773436695787188951\
                                  685343262506034537775941755001\
                                  87360389116729240", 10).unwrap();
    let y = BigInt::parse_bytes(b"326705100207588169780830851305\
                                  070431844712733806592432759389\
                                  04335757337482424", 10).unwrap();
    let seven = ToBigInt::to_bigint(&7).unwrap();

    let result : BigInt = (&x * &x * &x + &seven - &y * &y) % p;
    assert!(result.is_zero());
}
