# Day 21: Reading an account balance in Rust: address(account).balance in Solana

[Day 21](https://www.rareskills.io/post/solana-get-account-balance) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

- Anchor checks the account being read by default to see if the program owns the account.
- `UncheckedAccount` bypasses the safety check with reading accounts owned by the program.
- `/// CHECK: ...` doc comment is required by Anchor when bypassing safety checks like using `UncheckedAccount`.
- Not all `SOL` in an account is spendable, a portion of the balance must be maintained to be `rent exempt`.

## References

- [UncheckedAccount](https://docs.rs/anchor-lang/latest/anchor_lang/accounts/unchecked_account/struct.UncheckedAccount.html)
- [Anchor Accounts Struct Safety Checks](https://book.anchor-lang.com/anchor_in_depth/the_accounts_struct.html#safety-checks)
- [Sealevel Attacks (common attacks)](https://github.com/coral-xyz/sealevel-attacks)
