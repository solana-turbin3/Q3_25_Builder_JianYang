import {
    Connection, 
    Keypair,
    sendAndConfirmTransaction,
    PublicKey,
    Transaction, 
    LAMPORTS_PER_SOL, 
    SystemProgram,

 } from "@solana/web3.js";

import wallet from "./dev-wallet.json";
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

//turbine key
const to = new PublicKey("AkXiNtkzkknE5RjDotbpNyP9bsejVSffSEud5wygDscK");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        const balance = await connection.getBalance(from.publicKey)
        const tx = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL/10,
            })
        );
        tx.recentBlockhash = (await
            connection.getLatestBlockhash('confirmed')).blockhash;
            tx.feePayer = from.publicKey;

            const fee= (await connection.getFeeForMessage(tx.compileMessage(),
        'confirmed')).value || 0;

        tx.instructions.pop();
        tx.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee,
            })
        );

        const signature = await sendAndConfirmTransaction(
            connection,
            tx,
            [from]
        );
        console.log(`Success! Check tx here:: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();