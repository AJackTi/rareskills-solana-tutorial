# Day 5: Solana programs are upgradeable and do not have constructors

[Day 5](https://www.rareskills.io/post/solana-anchor-deploy) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- Solana programs don't have `constructors`.
- Solana programs are `mutable (upgradeable)` by default.
  - Solana doesn't need `delegatecall`, since programs can be upgraded.
- Solana programs don't have `immutable variables`.

## References

- [anchor deploy](https://www.anchor-lang.com/docs/cli#deploy)
- [Solana deploy "account data too small for instruction"](https://stackoverflow.com/questions/71267943/solana-deploy-account-data-too-small-for-instruction)
