import ForecastMarketIDL from "../target/idl/forecast_market.json";
import { ForecastMarketProgram } from "../sdk/forecast-market-program";
import {
  Connection,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  PublicKeyInitData,
  Keypair,
  clusterApiUrl,
} from "@solana/web3.js";
import { owner } from "./wallet";
import {
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  transfer,
} from "@solana/spl-token";
import { ForecastMarket } from "../target/types/forecast_market";
import { BN } from "@coral-xyz/anchor";
// import * as wormhole from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";
import { deriveAddress } from "@certusone/wormhole-sdk/lib/cjs/solana";
import { CONTRACTS } from "@certusone/wormhole-sdk";

export const NETWORK = "TESTNET";

export const WORMHOLE_CONTRACTS = CONTRACTS[NETWORK];
export const CORE_BRIDGE_PID = new PublicKey(WORMHOLE_CONTRACTS.solana.core);
export const TOKEN_BRIDGE_PID = new PublicKey(
  WORMHOLE_CONTRACTS.solana.token_bridge
);
// const args = process.argv.slice(2);

// const url = args.indexOf("--url");

function deriveWormholeMessageKey(
  programId: PublicKeyInitData,
  sequence: bigint
) {
  return deriveAddress(
    [
      Buffer.from("sent"),
      (() => {
        const buf = Buffer.alloc(8);
        buf.writeBigUInt64LE(sequence);
        return buf;
      })(),
    ],
    programId
  );
}

async function main() {
  // let rpcUrl = args[url + 1];

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const forecastMarketProgram = new ForecastMarketProgram(
    ForecastMarketIDL as ForecastMarket,
    connection
  );

  console.log(
    "Forecast Market Program: ",
    forecastMarketProgram.program.programId.toString()
  );

  // const configData = await forecastMarketProgram.getConfigData();

  // console.log("configData: ", configData.owner.toString());
  const payer = Keypair.fromSecretKey(
    Uint8Array.from([
      200, 189, 107, 206, 200, 244, 45, 23, 174, 117, 1, 220, 28, 164, 226, 216,
      67, 155, 93, 112, 85, 207, 121, 141, 122, 56, 191, 206, 106, 4, 212, 36,
      254, 63, 161, 96, 198, 101, 16, 201, 36, 74, 208, 186, 91, 183, 107, 176,
      109, 152, 173, 63, 232, 53, 196, 104, 77, 133, 217, 73, 242, 63, 246, 132,
    ])
  );
  const HELLO_WORLD_PID = new PublicKey(
    "GSo9DCHgno8P6GrvTP5sFVH3rBTicQYGwXxeF39Tymfu"
  );
  // const marketTx = await forecastMarketProgram.draftMarket(
  //   payer.publicKey,
  //   payer.publicKey,
  //   new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),
  //   new BN(122),
  //   "Test with legacy",
  //   new BN("0"),
  //   new BN("0"),
  //   new BN("0")
  // );
  // const transactionMarketTx = new Transaction().add(marketTx);
  // const createDraftMarket = await sendAndConfirmTransaction(
  //   connection,
  //   transactionMarketTx,
  //   [payer]
  // );
  // console.log("transactionMarketTx:", createDraftMarket);
  // const addAnswerKeys = await forecastMarketProgram.addAnswerKeys(
  //   payer.publicKey,
  //   new BN(122),
  //   [new BN(1), new BN(2), new BN(3)]
  // );
  // const transactionAddAnswerKeys = new Transaction().add(addAnswerKeys);
  // const createAddAnswerKeys = await sendAndConfirmTransaction(
  //   connection,
  //   transactionAddAnswerKeys,
  //   [payer]
  // );
  // console.log("transactionAddAnswerKeys:", createAddAnswerKeys);
  // const approveMarket = await forecastMarketProgram.approveMarket(
  //   payer.publicKey,
  //   new BN(122)
  // );
  // const transactionApproveMarket = new Transaction().add(approveMarket);
  // const createApproveMarket = await sendAndConfirmTransaction(
  //   connection,
  //   transactionApproveMarket,
  //   [payer]
  // );
  // console.log("transactionApproveMarket:", createApproveMarket);
  // const finnishMarket = await forecastMarketProgram.finishMarket(
  //   payer.publicKey,
  //   new BN(12)
  // );
  // const transaction = new Transaction().add(finnishMarket);
  // const createFinnishMarket = await sendAndConfirmTransaction(
  //   connection,
  //   transaction,
  //   [payer]
  // );
  // console.log("Transaction:", createFinnishMarket);
  const fetchMarket = await forecastMarketProgram.getMarketData(new BN(122));
  console.log("fetchMarket: ", fetchMarket);
  forecastMarketProgram.onBetCrossChainPlaced(async (event) => {
    console.log("event: ", event);
  });
  const fetchAnswers = await forecastMarketProgram.getAnswerData(new BN(122));
  console.log("fetchAnswers: ", fetchAnswers);
  // const vaa = (
  //   await wormhole.getPostedMessage(
  //     connection,
  //     deriveWormholeMessageKey(HELLO_WORLD_PID, 16n)
  //   )
  // ).message;

  // await forecastMarketProgram.betCrossChain(
  //   payer,
  //   1,
  //   "0x2304ab92d868067910ce8c60d6a7a5475d44d9fed5d5bfde6e8cbe1066c8a707",
  //   20,
  //   new BN(122),
  //   new BN(2),
  //   CORE_BRIDGE_PID
  // );
  await forecastMarketProgram.test();
}

main();
