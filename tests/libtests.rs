#[cfg(test)]
use toy_rsa::*;
use rug::integer::IsPrime;
use rug::Integer;

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
    assert_eq!(encrypt(349,9),4)
}
#[test]
fn test_encrypt_2(){
    assert_eq!(encrypt(214,42),16)
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

#[test]
fn count_genkey_loop(){
    let key = genkey();
    print!("received {:?}", (key.0,key.1));
    let p = Integer::from(key.0);
    let q = Integer::from(key.1);
    assert_eq!(p.is_probably_prime(15), IsPrime::Yes);
    assert_eq!(q.is_probably_prime(15), IsPrime::Yes);
}