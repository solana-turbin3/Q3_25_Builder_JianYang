import { Keypair, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";

import wallet from "./dev-wallet.json";
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        const txhash = await
        connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
        console.log(`Success! Check out you tx here: https://explorer.solana.com/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops. sth went wrong: ${e}`)
    }
})();