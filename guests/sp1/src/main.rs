#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::{attestation::Attestation, electra::beacon_state::BeaconState};

fn main() {
    println!("cycle-tracker-report-start: read-pre-state");
    let mut pre_state = sp1_zkvm::io::read::<BeaconState>();
    println!("cycle-tracker-report-end: read-pre-state");

    println!("cycle-tracker-report-start: read-attestation");
    let attestation = sp1_zkvm::io::read::<Attestation>();
    println!("cycle-tracker-report-end: read-attestation");

    println!("cycle-tracker-report-start: process-attestation");
    let _ = pre_state.process_attestation(&attestation);
    println!("cycle-tracker-report-end: process-attestation");
}
