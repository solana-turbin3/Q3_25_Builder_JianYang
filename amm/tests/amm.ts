// passing tests
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, Mint, mintTo, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAccount } from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import * as fs from 'fs';
describe("amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);
  
  const program = anchor.workspace.amm as Program<Amm>;

  const initializer = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(fs.readFileSync('./initializer.json', 'utf8')))
  );
  const user = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(fs.readFileSync('./user.json', 'utf8')))
  );
  // let user = Keypair.generate();
  let maxX = new anchor.BN(100_000_000);
  let maxY = new anchor.BN(100_000_000);
  let mintX: PublicKey;
  let mintY: PublicKey;
  let mintLp: PublicKey;
  let seed = new anchor.BN(69420);
  let userX: PublicKey;
  let userY: PublicKey;
  let vaultX: PublicKey;
  let vaultY: PublicKey;
  let config: PublicKey;
  let amount = new anchor.BN(1_000_000);
  let userLp: PublicKey;
  let bump: number;
  const mintAmount = 1_000_000_000;

  before(async () => {
    // await connection.requestAirdrop(initializer.publicKey, 10_000_000);
    // await new Promise(resolve => setTimeout(resolve, 4000));
    // await connection.requestAirdrop(user.publicKey, 10_000_000);
    // await new Promise(resolve => setTimeout(resolve, 4000));

    const configSeeds = [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)];
    [config, bump] = PublicKey.findProgramAddressSync(
      configSeeds,
      program.programId
    );

    [mintLp, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), config.toBuffer()],
      program.programId
    );

    mintX = await createMint(connection, initializer, initializer.publicKey, null, 6);
    mintY = await createMint(connection, initializer, initializer.publicKey, null, 6);

    userX = await createAssociatedTokenAccount(connection, user, mintX, user.publicKey);
    userY = await createAssociatedTokenAccount(connection, user, mintY, user.publicKey);

    

    vaultX = await getAssociatedTokenAddress(mintX, config, true );
    vaultY = await getAssociatedTokenAddress(mintY, config, true );
    await new Promise(resolve => setTimeout(resolve, 5000));

    await mintTo(connection, initializer, mintX, userX, initializer.publicKey, mintAmount);
    await mintTo(connection, initializer, mintY, userY, initializer.publicKey, mintAmount);

    userLp = await getAssociatedTokenAddress(mintLp, user.publicKey,);
  })

  it("Is initialized!", async () => {
    let fee = 1_000;
    // Add your test here.
    const tx = await program.methods.initialize(seed, fee, initializer.publicKey)
      .accounts({
        initializer: initializer.publicKey,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        vaultX: vaultX,
        vaultY: vaultY,
        config: config,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);
    const configAccount = await program.account.config.fetch(config);
    console.log(configAccount);
    console.log(`config address: ${config}`);
  });

  it("deposits tokens and mints lp tokens", async () => {
    
    const tx = await program.methods.deposit(amount, maxX, maxY)
      .accounts({
        user: user.publicKey,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        config: config,
        vaultX: vaultX,
        vaultY: vaultY,
        userX: userX,
        userY: userY,
        userLp: userLp,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    let userLpAccountBalance = (await getAccount(connection, userLp)).amount.toString();
    console.log(`userLpAccountBalance: ${userLpAccountBalance}`);
  });
});
