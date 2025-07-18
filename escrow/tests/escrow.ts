import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAccount,  createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, getAssociatedTokenAddressSync, mintTo, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { assert } from "chai";


describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const maker = Keypair.generate();
  const taker = Keypair.generate();

  const program = anchor.workspace.escrow as Program<Escrow>;

  // variables for the program
  let escrowPda: PublicKey;
  let vaultPda: PublicKey;
  const give = new anchor.BN(1_000_000_000_000);
  let receive = new anchor.BN(69_000_000_000);
  let mintA = Keypair.generate();
  let mintB = Keypair.generate();
  let bump: number;
  let seed = new anchor.BN(25);
  let makerAtaA: PublicKey;
  let makerAtaB: PublicKey;
  let takerAtaA: PublicKey;
  let takerAtaB: PublicKey;

  before(async () => {
    const makerAirdrop = await provider.connection.requestAirdrop(maker.publicKey, 100_000_000_000);
    await new Promise(resolve => setTimeout(resolve, 1000));
    const takerAirdrop = await provider.connection.requestAirdrop(taker.publicKey, 100_000_000_000);
    await new Promise(resolve => setTimeout(resolve, 1000));


    // creating mint_a
    await createMint(provider.connection, maker, maker.publicKey, null, 9, mintA);

    // creating mint_b
    await createMint(provider.connection, taker, taker.publicKey, null, 9, mintB);

    // escrowPDA
    [escrowPda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)], program.programId
    );

    makerAtaA = await createAssociatedTokenAccount(provider.connection, maker, mintA.publicKey, maker.publicKey);
    await mintTo(provider.connection, maker, mintA.publicKey, makerAtaA, maker, 1_000_000_000_000);
    
    takerAtaB = await createAssociatedTokenAccount(provider.connection, taker, mintB.publicKey, taker.publicKey);
    await mintTo(provider.connection, taker, mintB.publicKey, takerAtaB, taker, 69_000_000_000);
    vaultPda = await getAssociatedTokenAddressSync(mintA.publicKey, escrowPda, true, TOKEN_PROGRAM_ID);
    
    makerAtaB = await getAssociatedTokenAddressSync(mintB.publicKey, maker.publicKey, false, TOKEN_PROGRAM_ID);
    takerAtaA = await getAssociatedTokenAddressSync(mintA.publicKey, taker.publicKey, false, TOKEN_PROGRAM_ID);

  });

  it("make called", async () => {
    // Add your test here.
    const tx = await program.methods.make(seed, give, receive)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        // makerAtaA: makerAtaA,
        mintB: mintB.publicKey,
        // escrow: escrowPda,
        // vault: vaultPda,
        // systemProgram: SystemProgram,
        tokenProgram: TOKEN_PROGRAM_ID,
        // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,

      }).signers([maker]).rpc();
    console.log("Your transaction signature", tx);
    
    const makerAtaABefore = (await getAccount(provider.connection, makerAtaA)).amount;
    console.log(`INITIAL MAKER TOKEN_A BALANCE: ${makerAtaABefore}`);

    const vaultBalance = (await getAccount(provider.connection, vaultPda)).amount;
    console.log(`VAULT BALANCE TOKEN_A: ${vaultBalance}`);
    
    const takerAtaBBefore = (await getAccount(provider.connection, takerAtaB)).amount;
    console.log(`INITIAL TAKER TOKEN_B BALANCE:${takerAtaBBefore}`);

  });

  it("take called", async () => {
    const tx = await program.methods.take()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        mintB: mintB.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        escrow: escrowPda, 
        vault: vaultPda,
      }).signers([taker]).rpc();

    const makerAtaAAfter = (await getAccount(provider.connection, makerAtaA)).amount;
    console.log(`FINAL MAKER TOKEN_A BALANCE: ${makerAtaAAfter}`);

    const takerAtaBAfter = (await getAccount(provider.connection, takerAtaB)).amount;
    console.log(`FINAL TAKER TOKEN_B BALANCE:${takerAtaBAfter}`);
    
    const makerAtaBAfter = (await getAccount(provider.connection, makerAtaB)).amount;
    console.log(`FINAL MAKER TOKEN_B BALANCE: ${makerAtaBAfter}`);

    const takerAtaAAfter = (await getAccount(provider.connection, takerAtaA)).amount;
    console.log(`FINAL TAKER TOKEN_A BALANCE: ${takerAtaAAfter}`);
  });

  // it("refund called", async () => {
  //   const tx = await program.methods.refund(seed)
  //     .accounts({
  //       maker: maker.publicKey,
  //       mintA: mintA.publicKey,
  //       mintB: mintB.publicKey,
  //       makerAtaA: makerAtaA,
  //       escrow: escrowPda,
  //       vault: vaultPda,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID, 
  //       systemProgram: SystemProgram.programId,
  //     }).signers([maker]).rpc();

  //   const makerAtaAPostRefund = (await getAccount(provider.connection, makerAtaA)).amount;

  //   console.log(`MAKER ATA_A BALANCE POST REFUND: ${makerAtaAPostRefund}`);
  // })
});