# Day 17: Solana counter tutorial: reading and writing data to accounts

[Day 17](https://www.rareskills.io/post/solana-counter-program) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- A `Solana transaction` must specify in advance which accounts it will access.
  - Account storage is accessible via `Context` (`ctx.accounts.my_storage.x`);
- Use `mut` (mutable) keyword for `writing` (or reading and writing) to storage.
- View storage account with Solana CLI: `solana account <address>`
  ```
  Length: 16 (0x10) bytes
  0000:   1c f2 3b 85  43 19 31 28  ac 00 00 00  00 00 00 00   ..;.C.1(........
  ```
  - `First 8 bytes` are the `discriminator`.
  - `ac` is hex representation of `172` (170 incremented twice in the test case).

## References

- [Macro anchor_lang::Accounts](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)
- [Struct anchor_lang::context::Context](https://docs.rs/anchor-lang/latest/anchor_lang/context/struct.Context.html)
- [Anchor Discriminator](https://book.anchor-lang.com/anchor_bts/discriminator.html)
- [Solana CLI Reference and Usage](https://docs.solanalabs.com/cli/usage)
