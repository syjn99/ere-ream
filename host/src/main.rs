use std::env;

use ream_consensus::{attestation::Attestation, electra::beacon_state::BeaconState};
use zkvm_interface::{Input, ProverResourceType, zkVM};

mod zkvms;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up zkVM instance by cargo feature flags.
    let zkvm = zkvms::new_zkvm(ProverResourceType::Cpu)?;

    // Read inputs from files.
    let pre_state: BeaconState = utils::read_file(
        &std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
            .join("../assets/one_basic_attestation/pre.ssz_snappy"),
    );
    let attestation: Attestation = utils::read_file(
        &std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
            .join("../assets/one_basic_attestation/attestation.ssz_snappy"),
    );

    // Pass inputs to zkVM.
    let mut input = Input::new();
    input.write(pre_state);
    input.write(attestation);

    // Execute the program on the zkVM.
    // NOTE: execution doesn't return the output.
    let report = zkvm.execute(&input)?;

    // Print the execution report
    println!("Total cycles: {}", report.total_num_cycles);
    for (region, cycles) in report.region_cycles.iter() {
        println!("  Region: {}, Cycles: {}", region, cycles);
    }
    println!("Execution duration: {:?}", report.execution_duration);

    Ok(())
}
