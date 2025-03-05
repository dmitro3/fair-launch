import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BondingCurve } from "../target/types/bonding_curve"
import { Connection, PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction, ComputeBudgetProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js"
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, getAssociatedTokenAddress } from "@solana/spl-token"
import { expect } from "chai";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

const connection = new Connection("https://api.devnet.solana.com")

const CURVE_CONFIGURATION_SEED = "curve_configuration"
const POOL_SEED_PREFIX = "bonding_curve"
const SOL_VAULT_PREFIX = "liquidity_sol_vault"
function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

describe("bonding_curve", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const signer = provider.wallet as NodeWallet;
  const program = anchor.workspace.BondingCurve as Program<BondingCurve>;

  let mint: PublicKey
  let tokenAta: PublicKey
  const governance = Keypair.generate();
  const feeRecipient = Keypair.generate();




  it("Initialize the contract", async () => {

    try {
      const [curveConfig] = PublicKey.findProgramAddressSync(
        [Buffer.from(CURVE_CONFIGURATION_SEED)],
        program.programId
      )
      const fee = new BN(100);
      const initialQuorum = new BN(500);
      const tx = new Transaction()
        .add(
          await program.methods
            .initialize(fee, feeRecipient.publicKey, initialQuorum)
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
      const sig = await sendAndConfirmTransaction(connection, tx, [signer.payer], { skipPreflight: true })
      console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
      let curveConfigAccount = await program.account.curveConfiguration.fetch(curveConfig)
      console.log("Curve Configuration Data : ", curveConfigAccount)
    } catch (error) {
      console.log("Error in initialization :", error)
    }
  });

  it(" create bonding curve pool", async () => {

    try {
      // get existing TokenMint and TokenATA or we can create new token 
      mint = new PublicKey("EkScnsR7uVsTvo9kwNKdCYU3ikp6n6MeWPzZMYi1cUFe");

      tokenAta = await getAssociatedTokenAddress(
        new PublicKey("EkScnsR7uVsTvo9kwNKdCYU3ikp6n6MeWPzZMYi1cUFe"),
        signer.payer.publicKey,
      );
      console.log("token ATA: ", tokenAta.toBase58())


      const [bondingCurve] = PublicKey.findProgramAddressSync(
        [Buffer.from(POOL_SEED_PREFIX), mint.toBuffer()],
        program.programId
      )


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

});


