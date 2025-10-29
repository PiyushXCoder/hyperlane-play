import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hellosol } from "../target/types/hellosol";

describe("hellosol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.hellosol as Program<Hellosol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .receiveMessage(Buffer.from([104, 101, 108, 108, 111]))
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
