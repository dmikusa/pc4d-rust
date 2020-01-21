use std::env;
use sodiumoxide::crypto::secretbox;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} plaintext", env::args().nth(0).unwrap());
        println!("    plaintext: unencrypted data");
        std::process::exit(1);
    }

    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let message = env::args().nth(1).unwrap_or_default();
    
    let ciphertext = secretbox::seal(message.as_bytes(), &nonce, &key);
    let secretbox:Vec<u8> = nonce.as_ref().iter().map(|x| *x).chain(ciphertext.into_iter()).collect();

    println!("Nonce: {}", hex::encode(nonce.as_ref()));
    println!("Key: {}", hex::encode(key.as_ref()));
    println!("Box: {}", base64::encode(&secretbox));
    println!();
    println!("Decrypt with: debug/target/decrypt {} {}", hex::encode(key), base64::encode(&secretbox));
    println!();
}