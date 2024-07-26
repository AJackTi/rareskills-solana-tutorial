import { readFileSync } from 'fs';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';

import { Day24 } from '../target/types/day_24';

async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

/**
 * Function to convert a Uint8Array file to a Keypair.
 * @param filePath - Path to the JSON file containing the secret key array.
 * @returns {Keypair} - The Solana Keypair.
 */
function convertFileToKeypair(filePath: string): Keypair {
  // Read the file content
  const jsonData = readFileSync(filePath, "utf-8");

  // Parse the JSON content to get the array
  const keyArray: number[] = JSON.parse(jsonData);

  // Convert the array to a Uint8Array
  const secretKey = new Uint8Array(keyArray);

  // Create a Keypair from the Uint8Array
  const keypair = Keypair.fromSecretKey(secretKey);

  return keypair;
}

describe("day_24", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day24 as Program<Day24>;

  /*
  it("Is initialized!", async () => {
    // const newKeypair = anchor.web3.Keypair.generate();
    // await airdropSol(newKeypair.publicKey, 1e9); // 1 SOL

    // load the keypair
    const first = convertFileToKeypair(
      "/Users/ajackti/.config/solana/id2.json"
    );

    let seeds = [];

    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        myStorage: myStorage,
        signer: first.publicKey,
      })
      .signers([first])
      .rpc();
  });
  */

  /*
  it("initialize and update value", async () => {
    // load the keypair
    const first = convertFileToKeypair(
      "/Users/ajackti/.config/solana/id2.json"
    );

    let seeds = [];

    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        myStorage: myStorage,
        signer: first.publicKey,
      })
      .signers([first])
      .rpc();

    await program.methods
      .updateValue(new anchor.BN(10))
      .accounts({ myStorage: myStorage, fren: first.publicKey })
      .signers([first])
      .rpc();

    let value = await program.account.myStorage.fetch(myStorage);
    console.log(`value stored is ${value.x}`);
  });
  */

  /*
  it("initialize and update value 2", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();

    const airdropAliceTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropAliceTx);

    const airdropBobTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropBobTx);

    let seedsAlice = [alice.publicKey.toBytes()];
    const [playerAlice, _bumpA] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsAlice,
      program.programId
    );

    let seedsBob = [bob.publicKey.toBytes()];
    const [playerBob, _bumpB] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsBob,
      program.programId
    );

    // Alice and Bob initialize their accounts
    await program.methods
      .initialize1()
      .accounts({
        player: playerAlice,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize1()
      .accounts({
        player: playerBob,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    await program.methods
      .transferPoints1(5)
      .accounts({
        from: playerAlice,
        to: alice.publicKey,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    console.log(
      `Alice has ${
        (await program.account.player.fetch(playerAlice)).points
      } points`
    );

    console.log(
      `Bob has ${(await program.account.player.fetch(playerBob)).points} points`
    );
  });
  */

  it("initialize and update value 3", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();

    const airdropAliceTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropAliceTx);

    const airdropBobTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropBobTx);

    let seedsAlice = [alice.publicKey.toBytes()];
    const [playerAlice, _bumpA] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsAlice,
      program.programId
    );

    let seedsBob = [bob.publicKey.toBytes()];
    const [playerBob, _bumpB] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsBob,
      program.programId
    );

    // Alice and Bob initialize their accounts
    await program.methods
      .initialize1()
      .accounts({
        player: playerAlice,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize1()
      .accounts({
        player: playerBob,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    await program.methods
      .transferPoints2(5)
      .accounts({
        from: playerAlice,
        to: alice.publicKey,
        authority: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    console.log(
      `Alice has ${
        (await program.account.player.fetch(playerAlice)).points
      } points`
    );

    console.log(
      `Bob has ${(await program.account.player.fetch(playerBob)).points} points`
    );
  });
});
