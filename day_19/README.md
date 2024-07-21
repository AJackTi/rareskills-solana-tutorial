# Day 19: Creating “mappings” and “nested mapping” in Solana

[Day 19](https://www.rareskills.io/post/solana-solidity-mapping) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- `seeds=[]` in `init macro` behave as `keys` that can be used for mappings.
  - `keys` that are used by `init macro` need to be in an `instruction macro`.
- `seeds=[]` can have as many keys as the compute limit can support.
- `to_le_bytes()` means to `little endian` bytes.
- `to_be_bytes()` means to `big endian` bytes.

## References

- [What is Endianness? Big-Endian vs Little-Endian Explained with Examples](https://www.freecodecamp.org/news/what-is-endianness-big-endian-vs-little-endian/)
- [Macro anchor_lang::Accounts](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)
- [Rust to_le_bytes](https://doc.rust-lang.org/std/index.html?search=to_le_bytes)
- [Rust to_be_bytes](https://doc.rust-lang.org/std/index.html?search=to_be_bytes)
