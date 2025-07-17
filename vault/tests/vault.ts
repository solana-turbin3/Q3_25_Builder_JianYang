import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { Connection, Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert, use } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const user = provider.wallet;
  const program = anchor.workspace.vault as Program<Vault>;

  // defining variables for it blocks
  let vaultStatePda: PublicKey;
  let vaultStateBump: number;
  let vaultPda: PublicKey;
  let vaultBump: number;
  let amount = new anchor.BN(1_000_000_000);

  before(async () => {
    [vaultStatePda, vaultStateBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_state"), user.publicKey.toBuffer()], program.programId,
    );

    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()], program.programId
    );
  })

  // it("Initialized", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize()
  //     .accounts({
  //       user: user.publicKey,
  //       // vaultState: vaultStatePda,
  //       // vault: vaultPda
  //     })  
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("is deposited", async() => {
    let userBalanceBefore = await provider.connection.getBalance(user.publicKey);
    let vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
    const tx = await program.methods.deposit(amount)
      .accounts({
        user: user.publicKey,
        // vaultState: vaultStatePda,
        // vault: vaultPda,
      })
      .rpc();
    await new Promise(resolve => setTimeout(resolve, 1000));
    let userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    let vaultBalanceAfter = await provider.connection.getBalance(vaultPda);

    console.log(`user balance went from: ${userBalanceBefore} to ${userBalanceAfter}`);
    console.log(`vault balance went from ${vaultBalanceBefore} to ${vaultBalanceAfter}`);

    // getting the tx fee from tx details
    const txDetails = await provider.connection.getTransaction(tx, {commitment: "confirmed", maxSupportedTransactionVersion: 0});
    const fee = txDetails?.meta?.fee || 0;
    // console.log(`transaction fee: ${fee}`);
    // console.log(`these should be equal: ${userBalanceBefore}`);
    // console.log(`${userBalanceAfter + vaultBalanceAfter + fee}`);
    assert.equal(userBalanceBefore + vaultBalanceBefore, userBalanceAfter + vaultBalanceAfter + fee);
  });

  it("is withdrawn", async () => {
    let userBalanceBefore = await provider.connection.getBalance(user.publicKey);
    let vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
    const tx = await program.methods.withdraw(amount)
      .accounts({
        user: user.publicKey,
        // vaultState: vaultStatePda,
        // vault: vaultPda,
      })
      .rpc();
    await new Promise(resolve => setTimeout(resolve, 1000));
    let userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    let vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
    console.log(`user balance went from: ${userBalanceBefore} to ${userBalanceAfter}`);
    console.log(`vault balance went from ${vaultBalanceBefore} to ${vaultBalanceAfter}`);
    const txDetails = await provider.connection.getTransaction(tx, {commitment: "confirmed", maxSupportedTransactionVersion: 0});
    const fee = txDetails?.meta?.fee || 0;
    console.log(`tx fee: ${fee}`);
    assert.equal(userBalanceAfter + vaultBalanceAfter + fee, userBalanceBefore + vaultBalanceBefore);
  });

  // it("is closed", async() => {
  //   let userBalanceBefore = await provider.connection.getBalance(user.publicKey);
  //   let vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
  //   const tx = await program.methods.close()
  //   .accounts({
  //     user: user.publicKey,
  //   })
  //   .rpc();
  //   await new Promise(resolve => setTimeout(resolve, 1000));
  //   let userBalanceAfter = await provider.connection.getBalance(user.publicKey);
  //   const txDetails = await provider.connection.getTransaction(tx, {commitment: "confirmed", maxSupportedTransactionVersion: 0});
  //   const fee = txDetails?.meta?.fee || 0;
  //   assert.equal(userBalanceBefore + vaultBalanceBefore - fee, userBalanceAfter)
  // })
});
