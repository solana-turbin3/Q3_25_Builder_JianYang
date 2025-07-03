use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    message::Message,
};
use std::str::FromStr;

// const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
const RPC_URL: &str = "https://api.devnet.solana.com";

pub fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let rpc_client = RpcClient::new(RPC_URL);
    let to_pubkey = Pubkey::from_str("AkXiNtkzkknE5RjDotbpNyP9bsejVSffSEud5wygDscK").unwrap();

    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}

pub fn transfer_all_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let rpc_client = RpcClient::new(RPC_URL);
    let to_pubkey = Pubkey::from_str("AkXiNtkzkknE5RjDotbpNyP9bsejVSffSEud5wygDscK").unwrap();

    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
    let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );
    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send final transaction");
    println!("Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}
