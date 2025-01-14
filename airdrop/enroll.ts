import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbine3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com");

const github = Buffer.from("AirZED", "utf-8");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  preflightCommitment: "confirmed",
});

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);

(async () => {
  try {
    const txHash = await program.methods
      .complete(github)
      .accounts({ signer: keypair.publicKey })
      .signers([keypair])
      .rpc();

    console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
  } catch (e) {
    console.log("Oops, something went wrong!", e);
  }
})();
