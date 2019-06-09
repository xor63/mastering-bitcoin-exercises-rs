#[macro_use] extern crate hex_literal;

use rand_os::OsRng;
use rand_os::rand_core::RngCore;

use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;

const PRIME: [u8; 32] = hex!("FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF
                              FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F");

fn main() {
    let mut os_rng = OsRng::new().unwrap();
    let mut key = [0u8; 32];

    let max = GenericArray::clone_from_slice(&PRIME);

    loop {
        os_rng.fill_bytes(&mut key);
        let hash = Sha256::digest(&key);

        if hash < max {
            println!("Private Key: {:x}", hash);
            break;
        }
    }
}
