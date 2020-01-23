use sodiumoxide::crypto::auth;
use std::env;

fn main() {
    if env::args().count() != 4 {
        println!("Usage: {} key digest message", env::args().nth(0).unwrap());
        println!("    key: the key in hex format");
        println!("    digest: the digest in hex format");
        println!("    message: unencrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get the key
    let key = auth::Key::from_slice(
        &hex::decode(env::args().nth(1).unwrap_or_default()).expect("invalid hex"),
    )
    .expect("invalid key");

    // get the secret box
    let digest = auth::Tag::from_slice(
        &hex::decode(&env::args().nth(2).unwrap_or_default()).expect("invalid hex"),
    )
    .expect("invalid digest");

    // get message
    let message = env::args().nth(3).unwrap_or_default();

    // verify
    if auth::verify(&digest, message.as_bytes(), &key) {
        println!("Verified");
    } else {
        println!("NOT Verified");
    }
}
