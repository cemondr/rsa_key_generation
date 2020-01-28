use toy_rsa::*;
//use std::convert::{TryFrom, TryInto};

fn main(){
    
    println!("Example: 1");
    let private_key:(u32,u32) = (0xed23e6cd,0xf050a04d);
    let public_key :u64 = 16_040_792_824_636_804_009;
    let pre_encrypted_message = 12345;
    println!("pre_encrypted_message: {:?}",pre_encrypted_message );
    let encrypted_message = encrypt(public_key,pre_encrypted_message);
    println!("encrypted message: {:?}", encrypted_message);
    let decrypted_message = decrypt(private_key,encrypted_message);
    println!("decrypted message: {:?}",decrypted_message);
    println!();
    println!();

    println!("Example 2:");
    let private_key:(u32,u32) = genkey();
    let public_key: u64 = private_key.0 as u64 * private_key.1 as u64;
    let pre_encrypted_message = 10211991;
    println!("pre_encrypted_message_2: {:?}",pre_encrypted_message);
    let encrypted_message = encrypt(public_key,pre_encrypted_message);
    println!("encrypted message: {:?}",encrypted_message);
    let decrypted_message = decrypt(private_key,encrypted_message);
    println!("decrypted message: {:?}" , decrypted_message);

}