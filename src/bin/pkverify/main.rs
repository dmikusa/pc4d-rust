use sodiumoxide::crypto::sign;
use std::env;

fn main() {
    if env::args().count() != 3 {
        println!(
            "Usage: {} publicKey signedMessage",
            env::args().nth(0).unwrap()
        );
        println!("    publicKey: hex-encoded public key");
        println!("    signedMessage: base64-encoded signed message");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get sender's public key
    let signer_pk = sign::PublicKey::from_slice(
        &hex::decode(&env::args().nth(1).unwrap_or_default()).expect("invalid hex"),
    )
    .expect("invalid public key");

    // get message
    let signed_message =
        base64::decode(&env::args().nth(2).unwrap_or_default()).expect("invalid base64");

    // verify
    match sign::verify(&signed_message, &signer_pk) {
        Ok(_) => println!("Verified"),
        Err(_) => println!("NOT Verified"),
    }
}
