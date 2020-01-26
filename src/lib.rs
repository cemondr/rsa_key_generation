/* Provides ability to RSA encryption and decryption procedure  */

#![allow(dead_code)]
use toy_rsa_lib::*;

pub const EXP: u64 = 65_537;

/**calculates and returns the least common divisor of key(p-1,q-1)
 * 
 * 
 * tests follow immediately afterwards*/
pub fn lambda(p: u64, q: u64) -> u64{
    lcm(p-1,q-1)
}
/**encrypts by using the PowerMod function provided by the toy_rsa_lib  
 * 
 * 
 * tests follow immmediately afterwards*/
pub fn encrypt(key: u64, msg: u32) -> u64{
    modexp(key,EXP,msg as u64)
}
/**given a key pair and a message, it firsts calculates the inverse of 𝜆(p, q) mod E
 * and then calculates the power mod : msg^d mod (p ⋅ q) . returns decrypted message
 * 
 * 
 * tests follow immediately afterwards*/ 
pub fn decrypt(key:(u32,u32), msg:u64) -> u64{
    let x = key.0 as u64;
    let y = key.1 as u64;
    let d = modinverse(lambda(x,y), EXP);
    modexp(msg,d,x*y)
} 
/**Generates a pair of primes with the help of toy_rsa library
 * 
 * tests whether each element of the pair received is a prime*/
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