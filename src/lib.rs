#![allow(dead_code)]
use toy_rsa_lib::*;



fn lambda(p: u64, q: u64) -> u64{
    lcm(p-1,q-1)
}
fn encrypt(key: u64, msg: u64) -> u64{
    let e = 65537u64;
    modexp(key,e,msg)
}
#[test]
fn test_lambda(){
    assert_eq!(lambda(9,13),24)
}
#[test]
fn test_lambda_2(){
    assert_eq!(lambda(22,24),483)
}

#[test]
fn test_encrypt(){
    let e = 65537u64;
    assert_eq!(modexp(349,e,9),4)
}
#[test]
fn test_encrypt_2(){
    let e = 65537u64;
    assert_eq!(modexp(214,e,42),16)
}