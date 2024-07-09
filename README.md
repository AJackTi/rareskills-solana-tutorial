# rareskills-solana-tutorial

My attempt at learning [Solana](https://solana.com/) [program (smart contract)](https://solana.com/docs/core/programs) development through [RareSkill's Solana course](https://www.rareskills.io/solana-tutorial).

## Directory

- [Day 1: Hello World (Solana installation)](day-1/README.md)
- [Day 2: Function arguments, math, and arithmetic overflow](day-2/README.md)
- [Day 3: Anchor function magic and the Interface Definition Language](day-3/README.md)
- [Day 4: Solana reverts, errors, and basic access control](day-4/README.md)
- [Day 5: Where is the constructor? About anchor deploy](day-5/README.md)
- [Day 6: Solidity Translations to Rust and Solana](day-6/README.md)
- [Day 7: The unusual syntax of Rust](day-7/README.md)
- [Day 8: Understanding function-like macros in Rust](day-8/README.md)
- [Day 9: Rust Structs and Attribute-like and Custom Derive Macros](day-9/README.md)
- [Day 10: Translating Solidity function visibility and contract inheritance to Solana](day-10/README.md)
- [Day 11: Block variables in Solana: block.timestamp and block.number and others](day-11/README.md)
- [Day 12: Beyond the block: Sysvars](day-12/README.md)
- [Day 13: Native Programs: Sysvars](day-13/README.md)
- [Day 14: tx.origin, msg.sender, and onlyOwner in Solana](day-14/README.md)
- [Day 15: Transaction fees and compute units](day-15/README.md)
- [Day 16: Accounts in Solana](day-16/README.md)
- [Day 17: Writing to storage](day-17/README.md)
- [Day 18: Reading Accounts from Typescript — an alternative to public variables and view functions](day-18/README.md)
- [Day 19: Creating mappings and nested mappings in Solana](day-19/README.md)
- [Day 20: Cost of storage, maximum storage size, and account resizing](day-20/README.md)
- [Day 21: Reading an account balance in Rust: address(account).balance in Solana](day-21/README.md)
- [Day 22: More differences: modifiers, view pure, payable, and fallback in Solana](day-22/README.md)
- [Day 23: Building a payment splitter: “payable” and “msg.value” in Solana](day-23/README.md)
- [Day 24: Authorizing various wallets to write to an account: "Pranking tx.origin"](day-24/README.md)
- [Day 25: PDA vs Keypair Accounts](day-25/README.md)
- [Day 26: Understanding Account Ownership in Solana: Transferring SOL out of a PDA](day-26/README.md)
- [Day 27: init_if_needed and the Reinitialization Attack](day-27/README.md)
- [Day 28: Multicall in Solana: Batching Transactions](day-28/README.md)
- [Day 29: Owner vs Authority](day-29/README.md)
- [Day 30: Deleting Accounts and Closing Programs](day-30/README.md)

## References

- [RareSkill's Solana course](https://www.rareskills.io/solana-tutorial)
- [Solana](https://solana.com/)
- [What are Solana Programs?](https://solana.com/docs/core/programs)
- [Anchor](https://www.anchor-lang.com/)
- [60-days-of-solana](https://github.com/kshyun28/60-days-of-solana)

## Troubleshooting when deploy smart contract

````bash
❯ anchor deploy
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /Users/ajackti/.config/solana/id.json
Deploying program "day_2"...
Program path: /Users/ajackti/Downloads/Code/Solana/rareskills-solana-tutorial/day_2/target/deploy/day_2.so...
=============================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
=============================================================================
slide swear salmon castle awkward alter fossil swamp tonight note invest code
=============================================================================
To resume a deploy, pass the recovered keypair as the
[BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
=============================================================================
Error: Deploying program failed: RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: account data too small for instruction [3 log messages]
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.```
````

- Follow the below solution:

```bash
rm -rf target
```

```bash
anchor build
```

```bash
anchor deploy
```

```bash
anchor keys sync
```
