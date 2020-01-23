use sodiumoxide::crypto::box_;
use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} plaintext", env::args().nth(0).unwrap());
        println!("    plaintext: unencrypted data");
        std::process::exit(1);
    }

    sodiumoxide::init().expect("Cannot initialize Sodium Oxide/libsodium");

    // generate public/private keypairs
    let (sender_pk, sender_sk) = box_::gen_keypair();
    let (receiver_pk, receiver_sk) = box_::gen_keypair();

    // generate a nonce
    let nonce = box_::gen_nonce();

    // get message
    let message = env::args().nth(1).unwrap_or_default();

    // encrypt
    let ciphertext = box_::seal(message.as_bytes(), &nonce, &receiver_pk, &sender_sk);

    // combine the nonce & cipher text
    let mut secret: Vec<u8> = vec![];
    secret.extend(nonce.as_ref());
    secret.extend(ciphertext);

    println!("Sender public key: {}", base64::encode(&sender_pk));
    println!("Sender private key: {}", base64::encode(&sender_sk));
    println!("Recipient public key: {}", base64::encode(&receiver_pk));
    println!("Recipient private key: {}", base64::encode(&receiver_sk));
    println!("Nonce: {}", hex::encode(&nonce));
    println!("Box: {}", base64::encode(&secret));
    println!();
    println!(
        "Decrypt with: target/debug/pkdecrypt {} {} {}",
        base64::encode(&sender_pk),
        base64::encode(&receiver_sk),
        base64::encode(&secret)
    );
    println!();
}
