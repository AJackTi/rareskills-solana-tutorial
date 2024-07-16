# Day 14: Tx.origin, msg.sender, and onlyOwner in Solana: identifying the caller

[Day 14](https://www.rareskills.io/post/msg-sender-solana) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- Solana transactions can have `multiple signers`.
- Anchor automatically passess the `wallet account` in the `provider` as a `signer`.
- The `#[access_control]` attribute executes the given access control method before running the main instruction.
- `#[access_control]` can be used to implement functionality similar to `onlyOwner`.

## References

- [Attribute Macro anchor_lang::access_control](https://docs.rs/anchor-lang/latest/anchor_lang/attr.access_control.html)
