use sodiumoxide::crypto::pwhash;
use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} password", env::args().nth(0).unwrap());
        println!("    password: unencrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get password
    let password = env::args().nth(1).unwrap_or_default();

    // hash
    let hash = pwhash::pwhash(
        password.as_bytes(),
        pwhash::OPSLIMIT_INTERACTIVE,
        pwhash::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    println!("Password Hash: {}", hex::encode(&hash));
    println!();
    println!(
        "Verify with: target/debug/pwverify {} {:?}",
        base64::encode(&hash),
        password,
    );
    println!();
}
