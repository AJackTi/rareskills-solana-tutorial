# Day 2: Arithmetic and Basic Types in Solana and Rust

[Day 2](https://www.rareskills.io/post/rust-arithmetic-operators) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

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

### Solana Rust Arithmetic

- Solana Rust uses `u64` as the standard integer size.
- A `vector (Vec)` is Solana Rust's array.
- Solana has limited support for `floating point math` ([reference](https://solana.com/docs/programs/limitations#float-rust-types-support)).
- There are two ways to defend against `arithmetic overflow` in Solana:
  - by setting `overflow-checks=true` in the `Cargo.toml` file.
    - adding overflow checks increase the compute cost of the transaction.
  - by using `checked_*` operator.
    - `x.checked_add(y).unwrap()`
    - `x.checked_sub(y).unwrap()`
    - `x.checked_mul(y).unwrap()`
    - `x.checked_div(y).unwrap()`
- For getting the `power of y` of a number, the syntax is `x.pow(y)`.
- For getting the `square root` of a number, the syntax is `x.sqrt()`.
- For getting the `cube root` of a number, the syntax is `x.cbrt()`.
- For getting the `logarithm base 10` of a number, the syntax is `x.log10()`.

### Compute Units

- In Solana, `gas` is called a `compute unit`.
- A Solana transaction is limited to `200,000 compute units` by default.

## References

- [Rust Primitive Type `u64`](https://doc.rust-lang.org/std/primitive.u64.html)
- [Rust Primitive Type `f32`](https://doc.rust-lang.org/std/primitive.f32.html)
- [`u64` checked_add](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)
- [`u64` checked_sub](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_sub)
- [`u64` checked_mul](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul)
- [`u64` checked_div](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_div)
- [`f32` sqrt](https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt)
- [`f32` log10](https://doc.rust-lang.org/std/primitive.f32.html#method.log10)
