# Day 16: Initializing Accounts in Solana and Anchor

[Day 16](https://www.rareskills.io/post/solana-initialize-account) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

### Solana Accounts

- A `Solana program's bytecode` can be updated by the owner unless the program is marked as `immutable`.
  - Same behavior applies for `Solana storage`.
- Everything in Solana `is an account` (a `program account` or a `storage account`).
- A `program account` has `executable` set to `true`.
- A `storage account` has `executable` set to `false`.
- The `address` of the initialized accounts in Solana depends on:
  - the program that owns the storage account
  - and the `seeds`
- Accounts cannot be initialized twice.

### Solana Storage

- Solana storage are massive key-value stores.
  - key is a `base 58 encoded address`.
  - value is a data blob that can be up to `10 MB` in size.
- `Any program` can `read` storage variables, but only the `owner program` can `write` to it's own storage variables.
- A `Solana storage account` needs to be initialized (created) first before writing data to it.
  - It's possible to `initialize` and `write data` in one transaction, but this introduces security issues.
- Solana programs in Anchor treat all storage (or account data) as a `struct`.
  - Anchor `deserializes` and `serializes` account data into `structs` behind the scenes, when `reading` or `writing` data.
  - `#[account]` macro implements the `serialization`/`deserialization` of `structs` to `blob`, and vice versa.
- You're not required to use `structs` for Solana storage, but `structs` are a current convention.
- The `signer` pays for the `storage cost` (gas).

## References

- [Solana Accounts and Storing State](https://solana.com/docs/core/accounts)
- [Solana Account Model](https://solanacookbook.com/core-concepts/accounts.html#facts)
- [Macro anchor_lang::Accounts](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)
