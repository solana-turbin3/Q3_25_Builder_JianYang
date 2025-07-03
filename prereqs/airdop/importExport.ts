// @ts-ignore
import promptSync from "prompt-sync";
import bs58 from "bs58";

const prompt = promptSync();

console.log("Choose conversion:\n1. Base58 to Wallet JSON\n2. Wallet JSON to Base58");
const choice = prompt("Enter 1 or 2: ");

if (choice === "1") {
  const base58 = prompt("Paste your Phantom private key (Base58): ");
  const wallet = bs58.decode(base58);
  console.log("Wallet JSON array:\n", JSON.stringify(Array.from(wallet)));
} else if (choice === "2") {
  const input = prompt("Paste your wallet secret key JSON array: ");
  const walletArray = JSON.parse(input);
  const base58 = bs58.encode(Uint8Array.from(walletArray));
  console.log("Base58 private key:\n", base58);
} else {
  console.log("Invalid input.");
}

