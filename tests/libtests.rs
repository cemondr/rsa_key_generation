#[cfg(test)]
use toy_rsa::*;
use std::assert;
use toy_rsa_lib::*;


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
    assert_eq!(encrypt(0xde9c5816141c8ba9,12345),0x164e44b86776d497)
}
#[test]
fn test_encrypt_2(){
    assert_eq!(encrypt(9732371365385914997,10211991),6336136418123291215)
}

#[test]
fn test_decrypt(){
    let test_key = (0xed23e6cd,0xf050a04d);
    let test_message = 0x164e44b86776d497;
    //rustc infers type based on decrypt interface
    assert_eq!(decrypt(test_key,test_message), 12345)
}
#[test]
fn test_decrypt2(){
    let test_key = (3702622039, 2628507923);
    let test_message = 6336136418123291215;
    assert_eq!(decrypt(test_key,test_message),10211991)
}

#[test]
fn test_genkey(){
    let exp = 65537;
    let key = genkey();
    let lmbd = lambda(key.0 as u64,key.1 as u64);
    assert!(exp < lmbd , true);
    assert!(gcd(exp,lmbd) == 1, true)
    
}
