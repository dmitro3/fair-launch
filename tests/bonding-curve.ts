import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BondingCurve } from "../target/types/bonding_curve"
import { Connection, PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction, ComputeBudgetProgram, SYSVAR_RENT_PUBKEY, clusterApiUrl } from "@solana/web3.js"
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, getAssociatedTokenAddress } from "@solana/spl-token"
import { expect } from "chai";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as os from "os";
import { getPDAs, getKeypairFromFile } from "./utils";
const connection = new Connection(clusterApiUrl("devnet"))


describe("bonding_curve", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const signer = provider.wallet as NodeWallet;
  console.log("Signer address:", signer.publicKey.toBase58());
  const program = anchor.workspace.BondingCurve as Program<BondingCurve>;
  // make sure your key in the correct dir 
  let user = getKeypairFromFile(`${os.homedir()}/.config/solana/id2.json`);
  console.log("User address:", user.publicKey.toBase58());
  // get existing TokenMint and TokenATA or we can create new token 
  const mint = new PublicKey("BU38GveW5z5N61kuazeSJSPJCcQt9fn4SYZboBCxBVpz");
  //5ZoKnNrLwDw5FSgjuA7S7uSEsYPDHrhPzQ7bUTZxdtSa


  const governance = Keypair.generate();
  const feeRecipient = Keypair.generate();


  it("Initialize the contract", async () => {

    try {
      const { curveConfig } = await getPDAs(signer.payer.publicKey, mint)
      console.log("Curve Config:", curveConfig.toBase58())
      const fee = new BN(100);
      const initialQuorum = new BN(500);
      const targetLiquidity = new BN(10000000);
      const daoQuorum = new BN(500);
      // 0 is linear, 1 is quadratic
      const bondingCurveType = new BN(0);
      const tx = new Transaction()
        .add(
          await program.methods
            .initialize(fee, feeRecipient.publicKey, initialQuorum, targetLiquidity, governance.publicKey, daoQuorum, bondingCurveType)
            .accounts({
              dexConfigurationAccount: curveConfig,
              admin: signer.payer.publicKey,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true, commitment: "confirmed" })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      let curveConfigAccount = await program.account.curveConfiguration.fetch(curveConfig)
      console.log("Curve Configuration Data : ", curveConfigAccount)
    } catch (error) {
      console.log("Error in initialization :", error)
    }
  });

  it(" create bonding curve pool", async () => {

    try {


      const { bondingCurve } = await getPDAs(signer.payer.publicKey, mint)

      const tx = new Transaction()
        .add(
          await program.methods
            .createPool()
            .accounts({
              bondingCurveAccount: bondingCurve,
              tokenMint: mint,
              payer: signer.payer.publicKey,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      let bondingCurveAccount = await program.account.bondingCurve.fetch(bondingCurve)
      console.log("Bonding Curve Data : ", bondingCurveAccount)
    } catch (error) {
      console.log("Error in initialization :", error)
    }

  })



  it(" add liquidity to the pool", async () => {

    try {
      const { curveConfig, bondingCurve, poolSolVault, poolTokenAccount, userTokenAccount } = await getPDAs(signer.payer.publicKey, mint)
      const tx = new Transaction()
        .add(
          await program.methods
            .addLiquidity()
            .accounts({
              dexConfigurationAccount: curveConfig,
              bondingCurveAccount: bondingCurve,
              tokenMint: mint,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
              poolSolVault: poolSolVault,
              poolTokenAccount: poolTokenAccount,
              userTokenAccount: userTokenAccount,
              user: signer.payer.publicKey,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      const userBalance = (await connection.getTokenAccountBalance(userTokenAccount)).value.uiAmount
      const poolBalance = (await connection.getTokenAccountBalance(poolTokenAccount)).value.uiAmount
      console.log("User Balance : ", userBalance)
      console.log("Pool Balance : ", poolBalance)
    } catch (error) {
      console.log("Error in add liquidity :", error)
    }
  })


  it(" buy from the pool", async () => {

    try {


      const { curveConfig, bondingCurve, poolSolVault, poolTokenAccount, userTokenAccount } = await getPDAs(signer.payer.publicKey, mint)

      const tx = new Transaction()
        .add(
          await program.methods
            .buy(new BN(1000000000))
            .accounts({
              dexConfigurationAccount: curveConfig,
              bondingCurveAccount: bondingCurve,
              tokenMint: mint,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
              poolSolVault: poolSolVault,
              poolTokenAccount: poolTokenAccount,
              userTokenAccount: userTokenAccount,
              user: signer.payer.publicKey,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      const userBalance = (await connection.getTokenAccountBalance(userTokenAccount)).value.uiAmount
      const poolBalance = (await connection.getTokenAccountBalance(poolTokenAccount)).value.uiAmount
      console.log("User Balance : ", userBalance)
      console.log("Pool Balance : ", poolBalance)
    } catch (error) {
      console.log("Error in buy from pool :", error)
    }
  })


  it(" sell from the pool", async () => {

    try {


      const { curveConfig, bondingCurve, poolSolVault, poolTokenAccount, userTokenAccount, bump } = await getPDAs(signer.payer.publicKey, mint)

      const tx = new Transaction()
        .add(
          await program.methods
            .sell(new BN(1000000), bump)
            .accounts({
              dexConfigurationAccount: curveConfig,
              bondingCurveAccount: bondingCurve,
              tokenMint: mint,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
              poolSolVault: poolSolVault,
              poolTokenAccount: poolTokenAccount,
              userTokenAccount: userTokenAccount,
              user: signer.payer.publicKey,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      const userBalance = (await connection.getTokenAccountBalance(userTokenAccount)).value.uiAmount
      const poolBalance = (await connection.getTokenAccountBalance(poolTokenAccount)).value.uiAmount
      console.log("User Balance : ", userBalance)
      console.log("Pool Balance : ", poolBalance)
    } catch (error) {
      console.log("Error in sell from pool :", error)
    }
  })


  it(" remove liquidity to the pool", async () => {

    try {
      const { curveConfig, bondingCurve, poolSolVault, poolTokenAccount, userTokenAccount, bump } = await getPDAs(signer.payer.publicKey, mint)
      const tx = new Transaction()
        .add(
          await program.methods
            .removeLiquidity(bump)
            .accounts({
              dexConfigurationAccount: curveConfig,
              bondingCurveAccount: bondingCurve,
              tokenMint: mint,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
              poolSolVault: poolSolVault,
              poolTokenAccount: poolTokenAccount,
              userTokenAccount: userTokenAccount,
              user: signer.payer.publicKey,
              rent: SYSVAR_RENT_PUBKEY,
              systemProgram: SystemProgram.programId
            })
            .instruction()
        )
      tx.feePayer = signer.payer.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      const userBalance = (await connection.getTokenAccountBalance(userTokenAccount)).value.uiAmount
      const poolBalance = (await connection.getTokenAccountBalance(poolTokenAccount)).value.uiAmount
      console.log("User Balance : ", userBalance)
      console.log("Pool Balance : ", poolBalance)
    } catch (error) {
      console.log("Error in add liquidity :", error)
    }
  })


});


