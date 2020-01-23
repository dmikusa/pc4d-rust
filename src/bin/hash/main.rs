use digest::{ExtendableOutput, Input};
use sha3::Shake256;
use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} message", env::args().nth(0).unwrap());
        println!("    message: message to hash");
        std::process::exit(1);
    }

    // get message
    let message = env::args().nth(1).unwrap_or_default();

    // Perform Shake 256 Hash and get 64-bits of output
    let mut hasher = Shake256::default();
    hasher.input(message);
    let hash = hasher.vec_result(64);

    println!("Hash: {}", hex::encode(hash));
}
