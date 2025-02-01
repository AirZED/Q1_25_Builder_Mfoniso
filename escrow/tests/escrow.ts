import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";

// my deployed proramId
// DjLVzRa7F44qF4SY6KJDMNxAY93u1VH7iGV2Y6ERSt4F

describe("escrow", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.EscrowQ4 as Program<Escrow>;

  // accounts that we need
  // maker
  // mint_a
  // mint_b
  // maker_ata_a
  // escrow
  // vault
  // associated_token_program
  // system_program

  it("Making escrow!", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(new anchor.BN(1), new anchor.BN(1), new anchor.BN(1))
      .accounts({
        maker: anchor.web3.Keypair.generate().publicKey,
        mintA: anchor.web3.Keypair.generate().publicKey,
        mintB: anchor.web3.SystemProgram.programId,
      })
      .signers([anchor.web3.Keypair.generate()])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
