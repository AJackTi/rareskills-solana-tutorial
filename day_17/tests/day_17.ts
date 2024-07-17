import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day17 } from '../target/types/day_17';

describe("day_17", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day17 as Program<Day17>;

  it("set", async () => {
    const seeds = [];
    const [myStorage, _bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        seeds,
        program.programId
      );
    console.log("the storage account address is", myStorage.toBase58());

    const txInit = await program.methods
      .initialize()
      .accounts({ myStorage: myStorage })
      .rpc();
    console.log(
      `Init transaction signature: https://solscan.io/tx/${txInit}?cluster=devnet`
    );

    const txSet = await program.methods
      .set(new anchor.BN(1), new anchor.BN(2), new anchor.BN(3))
      .accounts({ myStorage: myStorage })
      .rpc();
    console.log(
      `Set transaction signature: https://solscan.io/tx/${txSet}?cluster=devnet`
    );

    const txPrintX = await program.methods
      .printX()
      .accounts({ myStorage })
      .rpc();
    console.log(
      `Print X transaction signature: https://solscan.io/tx/${txPrintX}?cluster=devnet`
    );

    const txIncrement = await program.methods
      .incrementX()
      .accounts({ myStorage })
      .rpc();
    console.log(
      `Increment transaction signature: https://solscan.io/tx/${txIncrement}?cluster=devnet`
    );

    const txPrintXAgain = await program.methods
      .printX()
      .accounts({ myStorage })
      .rpc();
    console.log(
      `Print X transaction signature: https://solscan.io/tx/${txPrintXAgain}?cluster=devnet`
    );
  });
});
