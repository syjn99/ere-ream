# Ream on ERE!

## Prepare

You should install all necessary dependencies for each zkVMs. Find out missing SDK in `eth-act/ere` repository ([link](https://github.com/eth-act/ere/tree/master/scripts/sdk_installers)).

## Execute

> [!Warning]
> As Zisk is not explictly supported in macOS (See [release note](https://github.com/0xPolygonHermez/zisk/releases/tag/v0.9.0) of `v0.9.0`), I haven't been able to test Zisk.

```bash
# Select zkVM with Rust Features in compile time.
make run_<sp1,risc0,openvm,pico,zisk>
```
