import fs from 'fs';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

let idl = JSON.parse(fs.readFileSync("target/idl/day_5.json", "utf-8"));

describe("day_5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "3Sk4JfmjMsoDwT8UZLqg2W3LQqtpc2KzQiieAUZnHv19";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
