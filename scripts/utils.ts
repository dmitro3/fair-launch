import { PublicKey } from "@solana/web3.js";

export function deserializeBondingCurve(data) {

    const offset = 8; // Adjust to 0 if no discriminator is used
  
    let currentOffset = offset;
  
    // creator: Pubkey (32 bytes)
    const creator = new PublicKey(data.slice(currentOffset, currentOffset + 32));
    currentOffset += 32;
  
    // total_supply: u64 (8 bytes, little-endian)
    const totalSupply = data.readBigUInt64LE(currentOffset);
    currentOffset += 8;
  
    // reserve_balance: u64 (8 bytes, little-endian)
    const reserveBalance = data.readBigUInt64LE(currentOffset);
    currentOffset += 8;
  
    // reserve_token: u64 (8 bytes, little-endian)
    const reserveToken = data.readBigUInt64LE(currentOffset);
    currentOffset += 8;
  
    // token: Pubkey (32 bytes)
    const token = new PublicKey(data.slice(currentOffset, currentOffset + 32));
    currentOffset += 32;
  
    // reserve_ratio: u16 (2 bytes, little-endian)
    const reserveRatio = data.readUInt16LE(currentOffset);
    currentOffset += 2;
  
    // bump: u8 (1 byte)
    const bump = data.readUInt8(currentOffset);
    currentOffset += 1;
  
    return {
      creator: creator.toBase58(),
      totalSupply: Number(totalSupply), // Convert BigInt to Number (if safe, else keep as BigInt)
      reserveBalance: Number(reserveBalance),
      reserveToken: Number(reserveToken),
      token: token.toBase58(),
      reserveRatio,
      bump,
    };
  }


export function deserializeCurveConfiguration(data) {
    if (data.length !== 104) {
      throw new Error(`Invalid account data length: expected 104 bytes, got ${data.length}`);
    }
  
  
    let offset = 8; // Skip the 8-byte discriminator
  
    // Read each field sequentially
    const fees = data.readDoubleLE(offset);
    offset += 8;
  
    const feeRecipient = new PublicKey(data.slice(offset, offset + 32)).toBase58();
    offset += 32;
  
    const initialQuorum = data.readBigUInt64LE(offset);
    offset += 8;
  
    const useDao = data.readUInt8(offset) !== 0;
    offset += 1;
  
    const governance = new PublicKey(data.slice(offset, offset + 32)).toBase58();
    offset += 32;
  
    const daoQuorum = data.readUInt16LE(offset);
    offset += 2;
  
    const lockedLiquidity = data.readUInt8(offset) !== 0;
    offset += 1;
  
    const targetLiquidity = data.readBigUInt64LE(offset);
    offset += 8;
  
    const feePercentage = data.readUInt16LE(offset);
    offset += 2;
  
    const feesEnabled = data.readUInt8(offset) !== 0;
    offset += 1;
  
    const bondingCurveType = data.readUInt8(offset);
    // offset += 1; // No need to increment further; we're at the end
  

    return {
      fees,                         // number (f64)
      feeRecipient,                 // string (base58)
      initialQuorum: initialQuorum.toString(), // string (u64 as string to avoid precision loss)
      useDao,                       // boolean
      governance,                   // string (base58)
      daoQuorum,                    // number (u16)
      lockedLiquidity,              // boolean
      targetLiquidity: targetLiquidity.toString(), // string (u64 as string)
      feePercentage,                // number (u16)
      feesEnabled,                  // boolean
      bondingCurveType,             // number (u8, enum variant index)
    };
  }

  
