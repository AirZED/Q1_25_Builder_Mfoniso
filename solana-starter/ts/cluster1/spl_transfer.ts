import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./wallet/wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("FsTMt4Dii382RVLSKYk1c55bugo2bh1Qpx3V2tDJTBGJ");

// Recipient address
const to = new PublicKey("MovHj25KabjUuoYRGMWHsGxHjb1JgCLdefbVrPFQwwJ");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    // Get the token account of the toWallet address, and if it does not exist, create it
    const ata_to = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );

    // Transfer the new token to the "toTokenAccount" we just created

    const transferTx = await transfer(
      connection,
      keypair,
      ata.address,
      ata_to.address,
      keypair,
      1e6
    );

    console.log(`Your transfer txid: ${transferTx}`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
