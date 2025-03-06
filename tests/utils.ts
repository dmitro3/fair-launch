import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BondingCurve } from "../target/types/bonding_curve"
import { getAssociatedTokenAddress } from "@solana/spl-token";


const CURVE_CONFIGURATION_SEED = "curve_configuration"
const POOL_SEED_PREFIX = "bonding_curve"
const SOL_VAULT_PREFIX = "liquidity_sol_vault"


const program = anchor.workspace.BondingCurve as Program<BondingCurve>;



export async function getPDAs(user: PublicKey, mint: PublicKey){
  const [curveConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from(CURVE_CONFIGURATION_SEED)],
    program.programId
  );

  const [bondingCurve] = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED_PREFIX), mint.toBuffer()],
    program.programId
  );

  const [poolSolVault] = PublicKey.findProgramAddressSync(
    [Buffer.from(SOL_VAULT_PREFIX), mint.toBuffer()],
    program.programId
  );
  const poolTokenAccount = await getAssociatedTokenAddress(
    mint, bondingCurve, true
  )
  const userTokenAccount = await getAssociatedTokenAddress(
    mint, user, true
  )
  return {
    userTokenAccount,
    curveConfig,
    bondingCurve,
    poolSolVault,
    poolTokenAccount,
  };
}
