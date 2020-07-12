use sodiumoxide::crypto::pwhash;
use std::env;

fn main() {
    if env::args().count() != 3 {
        println!("Usage: {} hash", env::args().nth(0).unwrap());
        println!("    hash: hashed password");
        println!("    password: potential password");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get password hash
    let password_hash = pwhash::scryptsalsa208sha256::HashedPassword::from_slice(
        &base64::decode(env::args().nth(1).unwrap_or_default()).expect("invalid base64"),
    )
    .expect("invalid hashed password");

    // get the potential password
    let password = env::args().nth(2).unwrap_or_default();

    // verify
    if pwhash::pwhash_verify(&password_hash, password.as_bytes()) {
        println!("Verified");
    } else {
        println!("NOT Verified");
    }
}
