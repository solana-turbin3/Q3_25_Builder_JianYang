import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
// const mint = new PublicKey("CKV2oKfUA9kk5sMgCvX43BRXEnokmxi1HGn58KRexuuS");
const mint = new PublicKey("9s5pGkv8raLNpNaa5xkhpG2QNKQCsPQMpddipA3GR1yW");

// Recipient address
const to = new PublicKey("6KQH8KmUukLK7SXik9CQSUXtKbkRhqGAKUeqiThch4ea");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWalletAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWalletAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);
        console.log(`${toWalletAta.address}`);
        // Transfer the new token to the "toTokenAccount" we just created
        const txid = await transfer(connection, keypair, fromWalletAta.address, toWalletAta.address, keypair, 69_00_00);
        console.log(`Transfer Successful ${txid}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();