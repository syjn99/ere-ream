# Ream on ERE!

## Prepare

You should install all necessary dependencies for each zkVM vendors. Find out missing SDK in `eth-act/ere` repository ([link](https://github.com/eth-act/ere/tree/master/scripts/sdk_installers)).

## Execute

```bash
# Use SP1 as zkVM.
cargo run --bin host --features sp1

# Use Risc0 as zkVM.
cargo run --bin host --features risc0
```
