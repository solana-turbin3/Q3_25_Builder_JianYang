use solana_sdk::{
    signature::{read_keypair_file, Signer},
    hash::hash,
};

pub fn verify_signature() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let pubkey = keypair.pubkey();

    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }
}
