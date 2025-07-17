# Ream on ERE!

## Prepare

You should install all necessary dependencies for each zkVMs. Find out missing SDK in `eth-act/ere` repository ([link](https://github.com/eth-act/ere/tree/master/scripts/sdk_installers)).

## Execute

```bash
# Select zkVM with Rust Features in compile time.
cargo run --bin host --features <sp1, risc0, openvm>
```
