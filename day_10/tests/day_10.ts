import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day10 } from '../target/types/day_10';

describe("day_10", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day10 as Program<Day10>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });
});
