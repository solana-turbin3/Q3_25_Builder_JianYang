use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    system_program,
    message::Message,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

const TURBIN3_PROGRAM_ID: &str = "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
const MINT_COLLECTION: &str = "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2";
const MPL_CORE_PROGRAM: &str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";

pub fn submit_rs() {
    let rpc_client = RpcClient::new(RPC_URL.to_string());
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Failed to read wallet");
    let signer_pubkey = signer.pubkey();

    let turbin3_program_id = Pubkey::from_str(TURBIN3_PROGRAM_ID).unwrap();
    let mint_collection = Pubkey::from_str(MINT_COLLECTION).unwrap();
    let mpl_core_program = Pubkey::from_str(MPL_CORE_PROGRAM).unwrap();

    let (account_key, _) = Pubkey::find_program_address(
        &[b"prereqs", signer_pubkey.as_ref()],
        &turbin3_program_id,
    );

    let (authority_key, _) = Pubkey::find_program_address(
        &[b"collection", mint_collection.as_ref()],
        &turbin3_program_id,
    );

    let mint = Keypair::new();

    let instruction = Instruction {
        program_id: turbin3_program_id,
        accounts: vec![
            AccountMeta::new(signer_pubkey, true),
            AccountMeta::new(account_key, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(mint_collection, false),
            AccountMeta::new_readonly(authority_key, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: vec![77, 124, 82, 163, 21, 133, 181, 206], // submit_rs discriminator
    };

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer_pubkey),
        &[&signer, &mint],
        blockhash,
    );

    let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", sig);
}