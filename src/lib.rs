/* Provides ability to RSA encryption and decryption procedure  */

#![allow(dead_code)]
use toy_rsa_lib::*;

pub const EXP: u64 = 65_537;

/**calculates and returns the least common divisor of key(p-1,q-1)
*/
pub fn lambda(p: u64, q: u64) -> u64{
    lcm(p-1,q-1)
}
/**returns an encrypted u64 by using the PowerMod function provided by the toy_rsa_lib  
*/
pub fn encrypt(key: u64, msg: u32) -> u64{
    modexp(msg as u64,EXP,key)
}
/**given a key pair and a message, it firsts calculates the modinverse of E, 𝜆(p, q)  
 * and then calculates the power mod : msg^d mod (p ⋅ q) . returns decrypted message
*/ 
pub fn decrypt(key:(u32,u32), msg:u64) -> u64{
    let x = key.0 as u64;
    let y = key.1 as u64;
    let d = modinverse(EXP,lambda(x,y));
    modexp(msg,d,x*y)
} 
/**Generates a pair of primes between the range of 2^30 and 2^31 that satisfy:
 * EXP < lambda(p,q) AND lambda(p,q) has no common factors with EXP. returns a pair
*/
pub fn genkey() -> (u32,u32){
    let mut p = rsa_prime();
    let mut q = rsa_prime();
    loop {
        let lmbd = lambda(p as u64,q as u64);
        if EXP < lmbd && gcd(EXP,lmbd) ==1 {
            break;
        }
        p = rsa_prime();
        q = rsa_prime();
    }
    (p,q)
}

pub fn genkey1() -> u32{
    rsa_prime()
}