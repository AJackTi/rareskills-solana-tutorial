import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Day19 } from "../target/types/day_19";

describe("day_19", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day19 as Program<Day19>;

  it("initializeMapping and setMapping", async () => {
    const key = new anchor.BN(1);
    const value = new anchor.BN(2);

    const seeds = [key.toArrayLike(Buffer, "le", 8)];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods
      .initializeMapping(key)
      .accounts({ val: valueAccount })
      .rpc();
    await program.methods
      .setMapping(key, value)
      .accounts({ val: valueAccount })
      .rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);

    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );
  });

  it("initializeNestedMapping and setNestedMapping", async () => {
    const key1 = new anchor.BN(1);
    const key2 = new anchor.BN(2);
    const value = new anchor.BN(3);

    const seeds = [
      key1.toArrayLike(Buffer, "le", 8),
      key2.toArrayLike(Buffer, "le", 8),
    ];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods
      .initializeNestedMapping(key1, key2)
      .accounts({ val: valueAccount })
      .rpc();
    await program.methods
      .setNestedMapping(key1, key2, value)
      .accounts({ val: valueAccount })
      .rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);

    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );
  });

  it("initializeMultipleMapping and setMultipleMapping", async () => {
    const whichMap1 = new anchor.BN(1);
    const whichMap2 = new anchor.BN(11);
    const key1 = new anchor.BN(2);
    const key2 = new anchor.BN(3);
    const value1 = new anchor.BN(4);
    const key21 = new anchor.BN(12);
    const key22 = new anchor.BN(13);
    const value2 = new anchor.BN(2);

    const seeds = [
      whichMap1.toArrayLike(Buffer, "le", 8),
      key1.toArrayLike(Buffer, "le", 8),
      key2.toArrayLike(Buffer, "le", 8),
    ];

    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods
      .initializeMultipleMapping(whichMap1, key1, key2)
      .accounts({ val: valueAccount })
      .rpc();

    await program.methods
      .setMultipleMapping(whichMap1, key1, key2, value1)
      .accounts({ val: valueAccount })
      .rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);
    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );

    await program.methods
      .setMultipleMapping(whichMap2, key21, key22, value2)
      .accounts({ val: valueAccount })
      .rpc();

    result = await program.account.val.fetch(valueAccount);
    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );

    console.log(`Final result data: ${JSON.stringify(result)}`);
  });
});
