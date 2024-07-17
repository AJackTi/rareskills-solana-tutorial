import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Day16 } from "../target/types/day_16";

describe("day_16", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day16 as Program<Day16>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );
    console.log("the storage account address is", myStorage.toBase58());

    const tx = await program.methods
      .initialize()
      .accounts({ myStorage: myStorage })
      .rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });
});
