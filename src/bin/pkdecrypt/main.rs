use sodiumoxide::crypto::box_;
use std::env;

fn main() {
    if env::args().count() != 4 {
        println!(
            "Usage: {} senderPublicKey recipientPrivateKey box",
            env::args().nth(0).unwrap()
        );
        println!("    senderPublicKey: base-64 encoded public key");
        println!("    recipientPrivateKey: base64-encoded private key");
        println!("    box: base64 encoded encrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // get sender's public key
    let sender_pk = box_::PublicKey::from_slice(
        &base64::decode(&env::args().nth(1).unwrap_or_default()).expect("invalid base64"),
    )
    .expect("invalid public key");

    // get receipient's private key
    let receiver_sk = box_::SecretKey::from_slice(
        &base64::decode(&env::args().nth(2).unwrap_or_default()).expect("invalid base64"),
    )
    .expect("invalid private key");

    // get the secret box
    let secret = base64::decode(&env::args().nth(3).unwrap_or_default()).expect("invalid base64");

    // regenerate the nonce
    let nonce = box_::Nonce::from_slice(&secret[0..box_::NONCEBYTES]).expect("invalid nonce");
    let ciphertext = &secret[box_::NONCEBYTES..];

    // decrypt
    println!("Nonce: {}", hex::encode(&nonce));
    if let Ok(message) = box_::open(ciphertext, &nonce, &sender_pk, &receiver_sk) {
        println!(
            "Message: {}",
            String::from_utf8(message).expect("Invalid utf-8")
        );
    } else {
        println!("Decryption failed :(");
    }
}
