import { Connection, GetProgramAccountsFilter, PublicKey } from "@solana/web3.js";

const rpcEndpoint = 'https://api.devnet.solana.com';
const solanaConnection = new Connection(rpcEndpoint);

const walletToQuery = 'Yo8A62FyZT4goufRRhDU6ENy3pLSVWEgFxe2SQhn5u6'; 
async function getBondingCurveAccounts(wallet: string, solanaConnection: Connection) {
    const filters:GetProgramAccountsFilter[] = [
        {
          dataSize: 165,    //size of account (bytes)
        },
        {
          memcmp: {
            offset: 32,     //location of our query in the account (bytes)
            bytes: wallet,  //our search criteria, a base58 encoded string
          },            
        }];
    const accounts = await solanaConnection.getParsedProgramAccounts(
        new PublicKey("Cs6KviV6T3GB7Q7d8YX41LhRYhwaWJgyoSE8TWruFbB5"),
        {filters: filters}
    );
    // console.log(`Found ${accounts.length} token account(s) for wallet ${wallet}.`);
    // accounts.forEach((account, i) => {
    //     //Parse the account data
    //     const parsedAccountInfo:any = account.account.data;
    //     // const mintAddress:string = parsedAccountInfo["parsed"]["info"]["mint"];
    //     // const tokenBalance: number = parsedAccountInfo["parsed"]["info"]["tokenAmount"]["uiAmount"];
    //     // //Log results
    //     // console.log(`Token Account No. ${i + 1}: ${account.pubkey.toString()}`);
    //     // console.log(`--Token Mint: ${mintAddress}`);
    //     // console.log(`--Token Balance: ${tokenBalance}`);
    //     // console.log(parsedAccountInfo)
    // });
}
getBondingCurveAccounts(walletToQuery,solanaConnection);

