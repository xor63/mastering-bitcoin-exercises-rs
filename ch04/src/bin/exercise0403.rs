//! Solution for Exercise 4.3

use openssl::nid::Nid;
use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcPoint, PointConversionForm};
use openssl::error::ErrorStack;

fn get_ec_group() -> Result<EcGroup, ErrorStack> {
    EcGroup::from_curve_name(Nid::SECP256K1)
}

fn get_ec_point() -> Result<EcPoint, ErrorStack> {
    let group = get_ec_group()?;
    EcPoint::new(&group)
}

fn main() {
    // private key from the book
    let k = "1E99423A4ED27608A15A2616A2B0E9E52CED330AC530EDCC32C8FFC6A526AEDD";

    let group = get_ec_group().unwrap();
    let n = BigNum::from_hex_str(&k).unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    let mut p = get_ec_point().unwrap();
    p.mul_generator(&group, &n, &mut ctx).unwrap();

    let mut ctx = BigNumContext::new().unwrap();

    let form = PointConversionForm::COMPRESSED;
    let pub_key = p.to_bytes(&group, form, &mut ctx).unwrap();
    println!("Public Key: {:X?}", pub_key);
}
