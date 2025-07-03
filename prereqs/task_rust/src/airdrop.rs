use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{read_keypair_file, Signer},
};

// const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
const RPC_URL: &str = "https://api.devnet.solana.com";

pub fn claim_airdrop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(sig) => {
            println!("Success! Check your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
        }
        Err(err) => {
            println!("Airdrop failed: {}", err);
        }
    }
}
