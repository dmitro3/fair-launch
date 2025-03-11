import { Connection, GetProgramAccountsFilter, PublicKey } from "@solana/web3.js";
import { Buffer } from "buffer";
import { deserializeBondingCurve, deserializeCurveConfiguration } from "./utils";

const programId = new PublicKey("ChNYPWh7iahysHscCesR37994jyGtALBhWevtAqay233");


const connection = new Connection("https://api.devnet.solana.com", {
  commitment: "confirmed",
});

async function getCurveConfig() {
  const seeds = [Buffer.from("curve_configuration")];
  const [curveConfig] = PublicKey.findProgramAddressSync(seeds, programId);
  console.log("Curve Config PDA:", curveConfig.toBase58());
  const accountInfo = await connection.getAccountInfo(curveConfig);
  if (!accountInfo) {
    console.log("PDA account does not exist or has no data.");
    return;
  }
  const decodedData = deserializeCurveConfiguration(accountInfo.data);
  console.log("Decoded Curve Configuration Data:", decodedData);
}
  
async function getBondingCurveAccounts(mint: PublicKey) {


  const seeds = [Buffer.from("bonding_curve"), mint.toBuffer()];


  const [bondingCurve, bump] = await PublicKey.findProgramAddressSync(seeds, programId);

  console.log("PDA Address:", bondingCurve.toBase58());

  const accountInfo = await connection.getAccountInfo(bondingCurve);

  if (!accountInfo) {
    console.log("PDA account does not exist or has no data.");
    return;
  }

  const decodedData = deserializeBondingCurve(accountInfo.data);
  console.log("Decoded BondingCurve Data:", decodedData);

}

const mint = new PublicKey("BU38GveW5z5N61kuazeSJSPJCcQt9fn4SYZboBCxBVpz");



async function main() {
  await getCurveConfig();
  await getBondingCurveAccounts(mint);

}

main();








