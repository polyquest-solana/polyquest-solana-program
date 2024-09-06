import { BN, IdlAccounts, Program } from '@coral-xyz/anchor';
import {
  Connection,
  PublicKey,
  Transaction,
} from '@solana/web3.js';
import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from '@solana/spl-token';
import { ForecastExchange } from '../target/types/forecast_exchange';

export type ConfigData = IdlAccounts<ForecastExchange>['configAccount'];

export class ForecastExchangeProgram {
  constructor(
    public readonly idl: ForecastExchange,
    public readonly connection: Connection
  ) { }

  get program() {
    return new Program(this.idl, { connection: this.connection });
  }

  get accounts(): any {
    return this.program.account;
  }

  get configPDA(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    )[0];
  }

  /**
    * Adds an event listener for the 'bought' event.
    * @param handler - The function to handle the event.
    */
  onBoughtPoint(handler: (event: any) => void) {
    this.program.addEventListener('bought', handler)
  }

  /**
    * Adds an event listener for the 'sold' event.
    * @param handler - The function to handle the event.
    */
  onSellPoint(handler: (event: any) => void) {
    this.program.addEventListener('sold', handler)
  }

  /**
   * Initializes the forecast exchange program.
   * @param owner - The public key of the owner.
   * @param pointMint - The public key of the point mint.
   * @param tokenMint - The public key of the token mint.
   * @returns A transaction object for the initialization.
   */
  public async initialize(owner: PublicKey, pointMint: PublicKey, tokenMint: PublicKey): Promise<Transaction> {
    const tx = await this.program.methods
      .initialize(pointMint, tokenMint)
      .accountsPartial({
        owner: owner,
        configAccount: this.configPDA,
      })
      .transaction();
    return tx;
  }

  /**
   * Buys points using tokens.
   * @param user - The public key of the user.
   * @param amount - The amount of points to buy.
   * @returns A transaction object for buying points.
   */
  public async buyPoint(user: PublicKey, amount: BN): Promise<Transaction> {

    const configData = await this.getConfigData();

    const ownerPointAccount = await getAssociatedTokenAddress(configData.pointMint, configData.owner, true);

    const userTokenAccount = await getAssociatedTokenAddress(configData.tokenMint, user, true);

    const tx = await this.program.methods
      .buyPoint(amount)
      .accountsPartial({
        user: user,
        configAccount: this.configPDA,
        ownerPointAccount: ownerPointAccount,
        userTokenAccount: userTokenAccount,
        tokenMint: configData.tokenMint,
        pointMint: configData.pointMint,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .transaction();
    return tx;
  }

  /**
   * Sells points to get tokens.
   * @param user - The public key of the user.
   * @param amount - The amount of points to sell.
   * @returns A transaction object for selling points.
   */
  public async sellPoint(user: PublicKey, amount: BN): Promise<Transaction> {

    const configData = await this.getConfigData();

    const userPointAccount = await getAssociatedTokenAddress(configData.pointMint, user, true);

    const userTokenAccount = await getAssociatedTokenAddress(configData.tokenMint, user, true);

    const ownerTokenAccount = await getAssociatedTokenAddress(configData.tokenMint, configData.owner, true);

    const tx = await this.program.methods
      .sellPoint(amount)
      .accountsPartial({
        user: user,
        configAccount: this.configPDA,
        userPointAccount: userPointAccount,
        userTokenAccount: userTokenAccount,
        ownerTokenAccount: ownerTokenAccount,
        tokenMint: configData.tokenMint,
        pointMint: configData.pointMint,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .transaction();
    return tx;
  }

  /**
   * Retrieves the configuration data.
   * @returns The configuration data.
   */
  public async getConfigData(): Promise<ConfigData> {
    const configPDA = this.configPDA;
    const configData = await this.accounts.configAccount.fetch(configPDA);
    return configData;
  }

}
