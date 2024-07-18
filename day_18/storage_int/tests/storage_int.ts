import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { StorageInt } from "../target/types/storage_int";

describe("storage_int", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StorageInt as Program<StorageInt>;

  it("Is initialized!", async () => {
    const seeds = [];

    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myStorage.toBase58());

    await program.methods.initialize().accounts({ myStorage }).rpc();

    await program.methods.set(new anchor.BN(1)).accounts({ myStorage }).rpc();

    // ***********************************
    // *** NEW CODE TO READ THE STRUCT ***
    // ***********************************

    let myStorageStruct = await program.account.myStorage.fetch(myStorage);
    console.log("The value of x is:", myStorageStruct.x.toString());

    let myStorageInfo = await anchor
      .getProvider()
      .connection.getAccountInfo(myStorage);
    console.log("the storage account info is", myStorageInfo);
  });
});
