import fs from "fs";

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { StorageBool } from "../target/types/storage_bool";

describe("storage_bool", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StorageBool as Program<StorageBool>;

  it("Is initialized!", async () => {
    const seeds = [];

    const [TrueOrFalse, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("The TrueOrFalse account address is", TrueOrFalse.toBase58());

    console.log("address:", program.programId.toBase58());

    await program.methods
      .initialize()
      .accounts({ trueOrFalse: TrueOrFalse })
      .rpc();

    await program.methods
      .setbool(true)
      .accounts({ trueOrFalse: TrueOrFalse })
      .rpc();
  });

  it("Read other contract", async () => {
    const otherProgramAddress = program.programId.toBase58();
    const otherProgramId = new anchor.web3.PublicKey(otherProgramAddress);

    const otherIdl = JSON.parse(
      fs.readFileSync("./target/idl/storage_bool.json", "utf8")
    );

    const otherProgram = new anchor.Program(otherIdl, otherProgramId);

    const seeds = [];
    const [trueOrFalseAcc, _bump] =
      anchor.web3.PublicKey.findProgramAddressSync(seeds, otherProgramId);
    let otherStorageStruct = await otherProgram.account.trueOrFalse.fetch(
      trueOrFalseAcc
    );

    console.log("The value of flag is:", otherStorageStruct.flag.toString());
  });
});
