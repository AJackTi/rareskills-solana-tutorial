# Day 3: Solana Anchor Program IDL

[Day 3](https://www.rareskills.io/post/anchor-idl) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

> **Note:** `anchor build` only affects `lib.rs` file.
>
> To test the `IDL` output of `non-empty-account.rs`, replace the contents of `lib.rs` with the contents from `non-empty-account.rs`.

## Notes

- When stopping transactions with invalid parameters:
  - `Ethereum` triggers a `revert`
  - while `Solana` returns an `error`.
- `require!` macro can be used as an alternative to `if statements`.
- Solana programs should always return an `Ok(())` or an `Error`.
  - All functions in Solana have a return type of `Result<()>`.
- In Anchor, errors are an enum with the `#[error_code]` attribute.

## Exercises

1. What pattern do you notice with the Error number? What happens to the error codes if you change the order of the errors in the Enum MyError?

   - The `Anchor error codes` start at `6000`, and increments by `1` from the first record in the `MyError enum`. (`6000`, `6001`, ...)

2. Use this code block which adds the new func and error to the existing code: Before you run this, what do you think the new error code will be?

   - The new error code will be `6002`, since there are three records in the `MyError enum`.

3. What happens if you put a msg! macro before the return error statements in a Solana program function? What happens if you replace `return err!` with `Ok(())`?

   - The `msg!` will not print when there's an error.
   - Replacing `return err!` with `Ok(())` will result in the transaction succeeding, printing the message `Will this print?"`.

## References

- [Errors](https://www.anchor-lang.com/docs/errors)
- [ErrorCode](https://docs.rs/anchor-lang/latest/anchor_lang/error/enum.ErrorCode.html)
