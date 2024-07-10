# Day 8: Rust function-like procedural Macros

[Day 8](https://www.rareskills.io/post/rust-function-like-macro) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- Rust functions cannot accept an arbitrary number of arguments.
- Rust `function-like macros` are identified by the presence of a `!` symbol.
  - examples are:
    - `println!()`
    - `msg!()`
- A Rust `macro` takes Rust code as input and programatically expands it into more Rust code.

## References

- [Rust std::io::stdout](https://doc.rust-lang.org/std/io/fn.stdout.html)
- [Macro solana_program::msg](https://docs.rs/solana-program/latest/solana_program/macro.msg.html)
- [Macro std::println](https://doc.rust-lang.org/std/macro.println.html)
