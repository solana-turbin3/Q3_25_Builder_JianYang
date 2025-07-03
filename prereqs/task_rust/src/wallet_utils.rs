use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use bs58;
use std::io::{self, BufRead};

#[test]
fn keygen() {
    use solana_sdk::signature::Keypair;

    let kp = Keypair::new();
    println!("generated keypair: {}", kp.pubkey().to_string());
    println!("Copy-paste the following JSON into dev-wallet.json:");

    let bytes = kp.to_bytes();
    print!("[");
    for (i, byte) in bytes.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", byte);
    }
    println!("]");
}

#[test]
fn base58_to_wallet() {
    println!("Input your private key as a base58 string:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file format is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input a JSON key");
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Your Base58-encoded private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}
