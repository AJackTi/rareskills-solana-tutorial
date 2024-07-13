import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day9 } from '../target/types/day_9';

describe("day_9", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day9 as Program<Day9>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("person", async () => {
    // Add your test here.
    const tx = await program.methods.person("AJackTi", 27).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("trait implement", async () => {
    // Add your test here.
    const tx = await program.methods.traitImplement(100.0, 50.0).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("my struct", async () => {
    // Add your test here.
    const tx = await program.methods.myStruct().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });
});
