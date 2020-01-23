use sodiumoxide::crypto::auth;
use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} plaintext", env::args().nth(0).unwrap());
        println!("    plaintext: unencrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // generate a key
    let key = auth::gen_key();

    // get message
    let message = env::args().nth(1).unwrap_or_default();

    // generate digest
    let digest = auth::authenticate(message.as_bytes(), &key);

    println!("Key: {}", hex::encode(&key));
    println!("Digest: {}", hex::encode(&digest));
    println!();
    println!(
        "Verify with: target/debug/verify {} {} {:?}",
        hex::encode(&key),
        hex::encode(&digest),
        message
    );
    println!();
}
