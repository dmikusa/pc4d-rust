use sodiumoxide::crypto::secretbox;
use std::env;

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
    let mut secret: Vec<u8> = vec![];
    secret.extend(nonce.as_ref());
    secret.extend(ciphertext);

    println!("Nonce: {}", hex::encode(&nonce));
    println!("Key: {}", hex::encode(&key));
    println!("Box: {}", base64::encode(&secret));
    println!();
    println!(
        "Decrypt with: target/debug/decrypt {} {}",
        hex::encode(&key),
        base64::encode(&secret)
    );
    println!();
}
