import { Keypair } from "@solana/web3.js";

// generate a new keypair
// 4nDrd4zTbwPZdpu8cxsDBawfkBB1CSTY5wXVKpXnWJSP
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);
console.log(`[${kp.secretKey}]`);