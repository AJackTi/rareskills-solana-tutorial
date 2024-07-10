import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day6 } from '../target/types/day_6';

describe("day_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Age checker", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Age checker v2", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerV2(new anchor.BN(35)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Age checker v3", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerV3(new anchor.BN(1)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("For loop", async () => {
    // Add your test here.
    const tx = await program.methods.forLoop(new anchor.BN(10)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("For loop step", async () => {
    // Add your test here.
    const tx = await program.methods.forLoopStep(new anchor.BN(10)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Fixed array", async () => {
    // Add your test here.
    const tx = await program.methods.fixedArray().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Dynamic array", async () => {
    // Add your test here.
    const tx = await program.methods.dynamicArray().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Hashmap", async () => {
    // Add your test here.
    const tx = await program.methods.hashMap("name", "ajackti").rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Struct func", async () => {
    // Add your test here.
    const tx = await program.methods
      .structFunc("Alice", new anchor.BN(20))
      .rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("constant", async () => {
    // Add your test here.
    const tx = await program.methods.constant().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("usize casting", async () => {
    // Add your test here.
    const tx = await program.methods.usizeCasting().rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("exercise", async () => {
    // Add your test here.
    const tx = await program.methods.exercise(new anchor.BN(10)).rpc();
    console.log(
      `Your transaction signature: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });
});
