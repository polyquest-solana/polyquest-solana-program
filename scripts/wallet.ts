import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

const testerPubkey = new PublicKey("7pceGYudQeSsf8aZ3y2wyH23k9k4xYcEskKgxJdKcDLg");
const testerPrivateKey = bs58.decode("4BU3ZSmLa5XhASXM2KZGZyC6KfFzzZDA7dpJcM9aerCfz8hyxMMxsa6TsmEuRjTBcKhFv2UwuAsx5V529kqLFvZi");

export const tester = new Keypair({
  publicKey: testerPubkey.toBytes(),
  secretKey: testerPrivateKey,
});

const ownerPubkey = new PublicKey("2f2D9o7tedW7J2Cn3c2ruh9rkcNdo3NUxw8rewxerszu");
const ownerPrivatekey = bs58.decode("3QDEv53wkJkQJEVCdazzjzZxcHvFrv6JmiF2WBMUPXErGKkk92AGWUsJufLVcjjwZu1qBGbpscGc8tg7N1bTtpVs");

export const owner = new Keypair({
  publicKey: ownerPubkey.toBytes(),
  secretKey: ownerPrivatekey,
});

export const serviceFeeAccount = new PublicKey('D7rNLjGP7kF9f9aHVWGGzicdRQbvv9mAPuaGRzJKQ2vy');
export const remainAccount = new PublicKey('D7rNLjGP7kF9f9aHVWGGzicdRQbvv9mAPuaGRzJKQ2vy');

export const newOwner = new PublicKey('H89ePJMMetaBBkHsjRYinnUAaoHsmsdg4LWh5XhLJqLY');
