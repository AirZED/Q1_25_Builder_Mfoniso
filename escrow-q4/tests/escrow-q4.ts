import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowQ4 } from "../target/types/escrow_q4";

describe("escrow-q4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EscrowQ4 as Program<EscrowQ4>;

  it("Making escrow!", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(new anchor.BN(1), new anchor.BN(1), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
