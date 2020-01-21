use std::env;
use sodiumoxide::crypto::secretbox;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} plaintext", env::args().nth(0).unwrap());
        println!("    plaintext: unencrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // generate a key
    let key = secretbox::gen_key();

    // generate a nonce
    let nonce = secretbox::gen_nonce();

    // get message
    let message = env::args().nth(1).unwrap_or_default();
    
    // encrypt
    let ciphertext = secretbox::seal(message.as_bytes(), &nonce, &key);

    // combine the nonce & cipher text
    let mut secretbox:Vec<u8> = vec![];
    secretbox.extend(nonce.as_ref());
    secretbox.extend(ciphertext);

    println!("Nonce: {}", hex::encode(&nonce));
    println!("Key: {}", hex::encode(&key));
    println!("Box: {}", base64::encode(&secretbox));
    println!();
    println!("Decrypt with: debug/target/decrypt {} {}", hex::encode(&key), base64::encode(&secretbox));
    println!();
}