import ForecastMarketIDL from "../target/idl/forecast_market.json";
import { ForecastMarketProgram } from "../sdk/forecast-market-program";
import {
  Connection,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  clusterApiUrl,
  Keypair,
} from "@solana/web3.js";
import { newOwner, owner, remainAccount, serviceFeeAccount } from "./wallet";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { ForecastMarket } from "../target/types/forecast_market";

const args = process.argv.slice(2);

const url = args.indexOf("--url");

async function main() {
  let tx: Transaction;
  const rewardApr = 0;

  let rpcUrl = args[url + 1];

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const forecastMarketProgram = new ForecastMarketProgram(
    ForecastMarketIDL as ForecastMarket,
    connection
  );

  console.log(
    "Forecast Market Program: ",
    forecastMarketProgram.program.programId.toString()
  );
  const rewardMint = new PublicKey(
    "GSo9DCHgno8P6GrvTP5sFVH3rBTicQYGwXxeF39Tymfu"
  );
  const owner = Keypair.fromSecretKey(
    Uint8Array.from([
      200, 189, 107, 206, 200, 244, 45, 23, 174, 117, 1, 220, 28, 164, 226, 216,
      67, 155, 93, 112, 85, 207, 121, 141, 122, 56, 191, 206, 106, 4, 212, 36,
      254, 63, 161, 96, 198, 101, 16, 201, 36, 74, 208, 186, 91, 183, 107, 176,
      109, 152, 173, 63, 232, 53, 196, 104, 77, 133, 217, 73, 242, 63, 246, 132,
    ])
  );
  //init forecast market program
  tx = await forecastMarketProgram.initialize(
    owner.publicKey,
    rewardMint,
    rewardApr
  );
  await sendAndConfirmTransaction(connection, tx, [owner]);

  // let configAccount = forecastMarketProgram.configPDA;
  // console.log('configAccount: ', configAccount.toString());

  // //set up fee service account and remain account
  // tx = await forecastMarketProgram.setAccount(owner.publicKey, serviceFeeAccount, remainAccount);
  // await sendAndConfirmTransaction(connection, tx, [owner]);

  // // update new owner
  // tx = await forecastMarketProgram.updateOwner(owner.publicKey, newOwner);
  // await sendAndConfirmTransaction(connection, tx, [owner]);
  // //set up token account
  // await getOrCreateAssociatedTokenAccount(connection, owner, rewardMint, remainAccount, true);

  // await getOrCreateAssociatedTokenAccount(connection, owner, rewardMint, serviceFeeAccount, true);

  // await getOrCreateAssociatedTokenAccount(connection, owner, rewardMint, forecastMarketProgram.configPDA, true);
}

main();
