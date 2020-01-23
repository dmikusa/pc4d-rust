use sodiumoxide::crypto::secretbox;
use std::env;

fn main() {
    if env::args().count() != 3 {
        println!("Usage: {} key box", env::args().nth(0).unwrap());
        println!("    key: hex-encoded 32 byte key");
        println!("    box: base64 encoded encrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get the key
    let key = secretbox::Key::from_slice(
        &hex::decode(env::args().nth(1).unwrap_or_default()).expect("invalid hex"),
    )
    .expect("invalid key");

    // get the secret box
    let secret = base64::decode(&env::args().nth(2).unwrap_or_default()).expect("invalid base64");

    // generate a nonce
    let nonce =
        secretbox::Nonce::from_slice(&secret[0..secretbox::NONCEBYTES]).expect("invalid nonce");
    let ciphertext = &secret[secretbox::NONCEBYTES..];

    // decrypt
    let message = secretbox::open(ciphertext, &nonce, &key).expect("decryption failed");
    println!("Nonce: {}", hex::encode(&nonce));
    println!(
        "Message: {}",
        String::from_utf8(message).expect("Invalid utf-8")
    );
}
