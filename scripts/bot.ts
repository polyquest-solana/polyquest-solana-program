import ForecastMarketIDL from '../target/idl/forecast_market.json';
import { ForecastMarketProgram } from "../sdk/forecast-market-program";
import { Connection, Transaction, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import { newOwner, owner, remainAccount, serviceFeeAccount } from './wallet';
import { getMint, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { ForecastMarket } from '../target/types/forecast_market';
import { BN } from '@coral-xyz/anchor';

const args = process.argv.slice(2);

const url = args.indexOf('--url');

async function main() {
  let tx: Transaction;
  const rewardApr = 1000;

  let rpcUrl = args[url + 1];

  const connection = new Connection(rpcUrl, 'confirmed');

  const forecastMarketProgram = new ForecastMarketProgram(ForecastMarketIDL as ForecastMarket, connection);
  
  console.log('Forecast Market Program: ', forecastMarketProgram.program.programId.toString());

  const marketId = new BN('23');

  const marketData = await forecastMarketProgram.getMarketData(marketId);
  
  const answerKey = await forecastMarketProgram.getAnswerData(marketId);
  console.log('answerKey: ', answerKey);

  console.log('marketData: ', marketData);

  const betMintInfo = await getMint(connection, marketData.betMint);
  
}

main();