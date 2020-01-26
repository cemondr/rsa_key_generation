/* Provides ability to RSA encryption and decryption procedure  */

#![allow(dead_code)]
use toy_rsa_lib::*;

pub const EXP: u64 = 65_537;

pub fn lambda(p: u64, q: u64) -> u64{
    lcm(p-1,q-1)
}
#[test]
fn test_lambda(){
    assert_eq!(lambda(9,13),24)
}
#[test]
fn test_lambda_2(){
    assert_eq!(lambda(22,24),483)
}


pub fn encrypt(key: u64, msg: u32) -> u64{
    modexp(key,EXP,msg as u64)
}
#[test]
fn test_encrypt(){
    assert_eq!(modexp(349,EXP,9),4)
}
#[test]
fn test_encrypt_2(){
    assert_eq!(modexp(214,EXP,42),16)
}

pub fn decrypt(key:(u32,u32), msg:u64) -> u64{
    let x = key.0 as u64;
    let y = key.1 as u64;
    let d = modinverse(lambda(x,y), EXP);
    modexp(msg,d,x*y)
} 
#[test]
fn test_decrypt(){
    let test_key = (4,10);
    let test_message = 23;
    //rustc infers type based on decrypt interface
    assert_eq!(decrypt(test_key,test_message), 9)
}
#[test]
fn test_decrypt2(){
    let test_key = (831,991);
    let test_message = 17;
    assert_eq!(decrypt(test_key,test_message),150586)
}


pub fn genkey() -> (u32,u32){
    let mut p = 2;
    let mut q = 2;
    loop {
        let lcm = lambda(p as u64,q as u64);
        if EXP< lcm && gcd(EXP,lcm) ==1 {
            break;
        }
        p = rsa_prime();
        q = rsa_prime();
    }
    (p,q)
}

#[test]
fn count_genkey_loop(){
    let (x,y) = genkey();
    print!("received {:?}", (x,y))
}