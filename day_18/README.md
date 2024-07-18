# Day 18: Read account data with Solana web3.js and Anchor

[Day 18](https://www.rareskills.io/post/solana-read-account-data) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- To read the storage of a Solana program build with Anchor, you need to have:
  - a `program address`
  - and its `IDL`
- Solana doesn't enforce how you serialize data in accounts.

## References

- [Solana getAccountInfo RPC Method](https://solana.com/docs/rpc/http/getaccountinfo)
- [Solana Cookbook: Serializing Data](https://solanacookbook.com/guides/serialization.html#setting-up-for-borsh-serialization)
