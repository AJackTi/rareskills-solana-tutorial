# Day 13: Solana logs, “events,” and transaction history

[Day 13](https://www.rareskills.io/post/solana-logs-transaction-history) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

## Setup

1. Replace your keypair in file `Anchor.toml`

- If you don't know how to config it. [Read here](https://solana.com/developers/guides/getstarted/setup-local-development)
- After setting it up successfully, check the path with:

```bash
echo "/Users/$USER/.config/solana/id.json"
```

- Replace the result in the file `Anchor.toml` like below:

```
  [provider]
  cluster = "Devnet"
  wallet = "/Users/ajackti/.config/solana/id.json"
```

2. Build Anchor program: `anchor build`
3. Sync program_id with Anchor key: `anchor keys sync`
4. Run tests: `anchor test`

## Notes

### Solana Events

- Events can be emitted using the `emit!` macro.
- Events are included in the Solana program's `IDL`.
- There are no indexed information in Solana (`index` field in the IDL does nothing).
- Events in Solana cannot be queried.
- Events in Solana can only be listened to as they occur.
- Events are preserved in the Solana block explorer.
- Logs are run by calling the system call [`sol_log_data`](https://docs.rs/solana-program/latest/src/solana_program/log.rs.html#116-124), which takes an argument of a `sequence of bytes`.

### Solana Transactions

- All transactions done by an address can be queried with [getSignaturesForAddress](https://solana.com/docs/rpc/http/getsignaturesforaddress) RPC function.
- Transaction content can be retrieved using [getParsedTransaction](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getParsedTransaction) RPC method.

## References

- [Macro anchor_lang::emit](https://docs.rs/anchor-lang/latest/anchor_lang/macro.emit.html)
- [anchor_lang::Event](https://docs.rs/anchor-lang/latest/anchor_lang/trait.Event.html)
