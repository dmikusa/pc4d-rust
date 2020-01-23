use sodiumoxide::crypto::sign;
use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} message", env::args().nth(0).unwrap());
        println!("    message: message to sign");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // generate public/private key
    let (public_key, private_key) = sign::gen_keypair();

    // get message
    let message = env::args().nth(1).unwrap_or_default();

    // sign it
    let signature = sign::sign(message.as_bytes(), &private_key);

    println!("Public Key: {}", hex::encode(&public_key));
    println!("Private Key: {}", hex::encode(&private_key));
    println!("Signature: {}", base64::encode(&signature));
    println!();
    println!(
        "Verify with: target/debug/pkverify {} {}",
        hex::encode(&public_key),
        base64::encode(&signature)
    );
    println!();
}
