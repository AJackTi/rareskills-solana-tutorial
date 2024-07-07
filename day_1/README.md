# Day 1: Solana Hello World (Installation and Troubleshooting)

[Day 1](https://www.rareskills.io/post/hello-world-solana) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

## Setup

Follow the [RareSkills tutorial](https://www.rareskills.io/post/hello-world-solana) for detailed steps.

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Solana CLI: `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
3. Install Anchor:

```
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

avm install latest
avm use latest
```

4. Initialize and build an Anchor Program

```
anchor init day_1 # I'm using a mac
cd day_1
anchor build
```

5. Deploy smart contract on devnet: `anchor deploy`
6. Run tests: `anchor test`
