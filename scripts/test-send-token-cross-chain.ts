import * as wormhole from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";
import { BN, Program } from "@coral-xyz/anchor";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  PublicKeyInitData,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import IDL from "../target/idl/forecast_exchange.json";
import { ForecastExchange } from "../target/types/forecast_exchange";
import {
  deriveAddress,
  getTokenBridgeDerivedAccounts,
  getTransferNativeWithPayloadCpiAccounts,
  getTransferWrappedWithPayloadCpiAccounts,
} from "@certusone/wormhole-sdk/lib/cjs/solana";
import {
  ChainId,
  CHAINS,
  CONTRACTS,
  parseTokenTransferPayload,
} from "@certusone/wormhole-sdk";
import { getAccount, getAssociatedTokenAddressSync } from "@solana/spl-token";
import {
  deriveEndpointKey,
  getWrappedMeta,
} from "@certusone/wormhole-sdk/lib/cjs/solana/tokenBridge";

const NETWORK = "TESTNET";

const WORMHOLE_CONTRACTS = CONTRACTS[NETWORK];
const CORE_BRIDGE_PID = new PublicKey(WORMHOLE_CONTRACTS.solana.core);
const TOKEN_BRIDGE_PID = new PublicKey(WORMHOLE_CONTRACTS.solana.token_bridge);
const BASE_TOKEN_BRIDGE_PID = WORMHOLE_CONTRACTS.base_sepolia.token_bridge;
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
const programId = new PublicKey("FVWe7wMvbWvnk1Vo4viZLejdK7oFiy5S5csLAXazbuQG");
function deriveTmpTokenAccountKey(
  programId: PublicKeyInitData,
  mint: PublicKeyInitData
) {
  return deriveAddress(
    [Buffer.from("tmp"), new PublicKey(mint).toBuffer()],
    programId
  );
}
function deriveTokenTransferMessageKey(
  programId: PublicKeyInitData,
  sequence: bigint
) {
  return deriveAddress(
    [
      Buffer.from("bridged"),
      (() => {
        const buf = Buffer.alloc(8);
        buf.writeBigUInt64LE(sequence);
        return buf;
      })(),
    ],
    programId
  );
}
export function deriveForeignContractKey(
  programId: PublicKey,
  chainId: ChainId
) {
  const seedPrefix = Buffer.from("foreign_contract");
  const chainBuffer = Buffer.alloc(2);
  chainBuffer.writeUInt16LE(chainId, 0); // chainId vào Buffer theo little endian
  console.log(chainBuffer);
  return deriveAddress([seedPrefix, chainBuffer], programId);
}

const getWormholeSequence = async () =>
  (
    await wormhole.getProgramSequenceTracker(
      connection,
      TOKEN_BRIDGE_PID,
      CORE_BRIDGE_PID
    )
  ).value() + 1n;
const verifyWormholeMessage = async (sequence: bigint) => {
  const payload = parseTokenTransferPayload(
    (
      await wormhole.getPostedMessage(
        connection,
        deriveTokenTransferMessageKey(programId, sequence)
      )
    ).message.payload
  ).tokenTransferPayload;
};

const verifyTmpTokenAccountDoesNotExist = async (mint: PublicKey) => {
  const tmpTokenAccountKey = deriveTmpTokenAccountKey(programId, mint);
  return getAccount(connection, tmpTokenAccountKey);
};

const getTokenBalance = async (tokenAccount: PublicKey) =>
  (await getAccount(connection, tokenAccount)).amount;

export function deriveSenderConfigKey(programId: PublicKeyInitData) {
  return deriveAddress([Buffer.from("sender")], programId);
}
const main_solana = async () => {
  const payer = Keypair.fromSecretKey(
    Uint8Array.from([
      200, 189, 107, 206, 200, 244, 45, 23, 174, 117, 1, 220, 28, 164, 226, 216,
      67, 155, 93, 112, 85, 207, 121, 141, 122, 56, 191, 206, 106, 4, 212, 36,
      254, 63, 161, 96, 198, 101, 16, 201, 36, 74, 208, 186, 91, 183, 107, 176,
      109, 152, 173, 63, 232, 53, 196, 104, 77, 133, 217, 73, 242, 63, 246, 132,
    ])
  );

  const program = new Program(IDL as ForecastExchange, {
    connection: connection,
  });
  const init = async () => {
    const tokenBridgeAccounts = getTokenBridgeDerivedAccounts(
      program.programId,
      TOKEN_BRIDGE_PID,
      CORE_BRIDGE_PID
    );
    console.log(
      deriveEndpointKey(
        TOKEN_BRIDGE_PID,
        CHAINS.base_sepolia,
        BASE_TOKEN_BRIDGE_PID
      )
    );
    const accounts = {
      owner: payer.publicKey,
      senderConfig: deriveSenderConfigKey(program.programId),
      tokenBridgeProgram: TOKEN_BRIDGE_PID,
      wormholeProgram: CORE_BRIDGE_PID,
      ...tokenBridgeAccounts,
    };
    const foreignContractAddress = Buffer.alloc(32, "deadbeef", "hex");
    const tx = await program.methods
      .initializeBridge(0, 100_000_000)
      .accounts(accounts)
      .instruction();
    const transaction = new Transaction().add(tx);

    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer] // Keypair sử dụng để ký giao dịch
    );
    console.log("Signature Init: ", signature);
  };
  // init();
  const transfer = async () => {
    let recipientAddress = new Uint8Array(32);
    recipientAddress.set(
      Buffer.from("0xeb055a6e41b7edc92951ace0fcbfb6ad460e0aaa".slice(2), "hex"),
      12
    );
    let recipientContract = new Uint8Array(32);
    recipientContract.set(
      Buffer.from("0x2649FedaaB993D98F36e509bf9a9b6908719F17C".slice(2), "hex"),
      12
    );
    const mint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    const tracker = await wormhole.getProgramSequenceTracker(
      connection,
      TOKEN_BRIDGE_PID,
      CORE_BRIDGE_PID
    );
    const recipientChain = CHAINS.base_sepolia;

    console.log("Sequence: ", tracker.sequence + 1n);

    const message = deriveTokenTransferMessageKey(
      programId,
      tracker.sequence + 1n
    );

    const fromTokenAccount = getAssociatedTokenAddressSync(
      mint,
      payer.publicKey
    );
    const tokenBridgeAccounts = getTransferNativeWithPayloadCpiAccounts(
      programId,
      TOKEN_BRIDGE_PID,
      CORE_BRIDGE_PID,
      payer.publicKey,
      message,
      fromTokenAccount,
      mint
    );
    const accounts = {
      config: deriveSenderConfigKey(programId),
      foreignContract: deriveForeignContractKey(programId, recipientChain),
      tmpTokenAccount: deriveTmpTokenAccountKey(programId, mint),
      tokenBridgeProgram: TOKEN_BRIDGE_PID,
      ...tokenBridgeAccounts,
    };
    const tx = await program.methods
      .transferCrossChain(
        0,
        new BN(123456),
        [...recipientAddress],
        recipientChain,
        [...recipientContract]
      )
      .accounts(accounts)
      .instruction();

    const transaction = new Transaction().add(tx);

    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer] // Keypair sử dụng để ký giao dịch
    );
    console.log("Signature Transfer: ", signature);
  };
  transfer();
  // init();
};
// main_solana();
// console.log()
//29523
//1
const test = async () => {
  const vaa = (
    await wormhole.getPostedMessage(
      connection,
      deriveTokenTransferMessageKey(programId, 29523n)
    )
  ).message;
  console.log(vaa);
  const payload = vaa.payload;
  console.log(
    parseTokenTransferPayload(payload)
      .tokenTransferPayload.subarray(1, 33)
      .toString("hex")
  );
  let recipientAddress = new Uint8Array(32);
  recipientAddress.set(
    Buffer.from("0xeb055a6e41b7edc92951ace0fcbfb6ad460e0aaa".slice(2), "hex"),
    12
  );
  console.log("Receiver Address:", recipientAddress);
  const emitterAddressHex = vaa.emitterAddress.toString("hex");
  console.log("Emitter Address:", emitterAddressHex);
};

test();
