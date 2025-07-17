# Ream on ERE!

This project is for show-casing [Ere](https://github.com/eth-act/ere) with [Ream](https://github.com/ReamLabs/ream) codebase. Each guest program runs [`process_attestation`](https://github.com/ethereum/consensus-specs/blob/dev/specs/electra/beacon-chain.md#modified-process_attestation) which is quite simple (and also includes BLS verification).

## Prepare

You should install all necessary dependencies for each zkVMs. Find out missing SDK in `eth-act/ere` repository ([link](https://github.com/eth-act/ere/tree/master/scripts/sdk_installers)).

## Execute

> [!Warning]
> As Zisk is not explictly supported in macOS (See [release note](https://github.com/0xPolygonHermez/zisk/releases/tag/v0.9.0) of `v0.9.0`), I haven't been able to test Zisk.

> [!Warning]
> Jolt is [not fully supported](https://github.com/eth-act/ere/blob/3da61c14cbefc2d15d50b982632af261277d1357/crates/ere-jolt/src/lib.rs#L54-L68) by Ere. `make run_jolt` will exit with a message.

```bash
# Select zkVM with Rust Features in compile time.
make run_<sp1,risc0,openvm,pico,zisk,jolt>
```

## Prove

You can modify `host/main.rs` like following:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...

    let (proof, report) = zkvm.prove(&input)?;
    println!("prove cost: {:?} s", report.proving_time.as_secs_f64());
    let is_verifier = zkvm.verify(&proof).is_ok();
    if is_verifier {
        println!("Executed successfully");
    } else {
        println!("Verification failed.");
    }
    Ok(())
}
```

But as BLS verification is included, proving process would take so much time in daily laptops like MacBook. We might improve this by leveraging precompiles.

## Notes

### About Report

```rust
/// ProgramExecutionReport produces information about a particular program
/// execution.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramExecutionReport {
    /// Total number of cycles for the entire workload execution.
    pub total_num_cycles: u64,
    /// Region-specific cycles, mapping region names (e.g., "setup", "compute") to their cycle counts.
    pub region_cycles: IndexMap<String, u64>,
    /// Execution duration.
    pub execution_duration: Duration,
}
```

Although `ProgramExecutionReport` has the fields, I'd like to note that not every zkVM supports feature corresponding to each field. For example, `region_cycles` is only available on SP1 as it supports a [cycle tracking feature](https://docs.succinct.xyz/docs/sp1/optimizing-programs/cycle-tracking).

## Acknowledgement

- [`eth-act/ere`](https://github.com/eth-act/ere): Great abstraction for various zkVMs. I really appreciate their effort for doing low level stuffs.
- [`SuccinctPaul/ere-zkvm-examples`](https://github.com/SuccinctPaul/ere-zkvm-examples): This project is inspired by Paul's example usage.