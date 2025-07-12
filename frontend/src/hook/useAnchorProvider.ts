import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import idlBondingCurve from "../contracts/IDLs/bonding_curve.json";
import { Keypair } from "@solana/web3.js";

export default function useAnchorProvider() {
  const anchorWallet = useAnchorWallet();
  const { connection } = useConnection();
  
  // Return null if wallet or connection is not available
  if (!connection || !anchorWallet) {
    return null;
  }

  const providerProgram = new anchor.AnchorProvider(
    connection,
    anchorWallet as any,
    {
      preflightCommitment: "confirmed",
    }
  );
  
  const program = new Program(
    idlBondingCurve as anchor.Idl,
    providerProgram as any
  );

  const governanceKeypair = Keypair.generate();
  const mintKeypair = Keypair.generate();
  
  return {
    connection,
    anchorWallet,
    providerProgram,
    program,
    governanceKeypair,
    mintKeypair,
  };
}