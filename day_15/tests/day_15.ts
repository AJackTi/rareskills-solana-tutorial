import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day15 } from '../target/types/day_15';

describe("day_15", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day15 as Program<Day15>;

  const defaultKeyPair = new anchor.web3.PublicKey(
    "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx"
  );

  it("Is initialized!", async () => {
    // log the keypair's initial balance
    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before:", bal_before);

    // call the initialize function of our program
    const tx = await program.methods.initialize().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );

    // log the keypair's balance after
    let bal_after = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("after:", bal_after);

    // log the difference
    console.log(
      "diff:",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );
  });
});
