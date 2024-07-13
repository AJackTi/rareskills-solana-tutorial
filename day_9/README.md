# Day 9: Rust Structs and Attribute-like and Custom Derive Macros

[Day 9](https://www.rareskills.io/post/rust-attribute-derive-macro) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- Rust `impl` are used to create `functions` that operate on a `struct`.
- Rust `Traits` enforces an `impl` to implement certain functions.
- An `attribute-like macro` takes in a `struct` and can completely rewrite it.
- A `derive macro` augments a struct with additional functions.
- `macros` allow Anchor to hide complexity.

## References

- [cargo new](https://doc.rust-lang.org/cargo/commands/cargo-new.html)
- [cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html)
- [Rust Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust `impl`](https://doc.rust-lang.org/std/keyword.impl.html)
- [Create solana_program](https://docs.rs/solana-program/latest/solana_program/)
- [Macro anchor_lang::program](https://docs.rs/anchor-lang/latest/anchor_lang/attr.program.html)
- [Derive Macro anchor_lang::Accounts](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)
