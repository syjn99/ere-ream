use ream_consensus::{attestation::Attestation, electra::beacon_state::BeaconState};
use risc0_zkvm::guest::env;

fn main() {
    let mut pre_state = env::read::<BeaconState>();
    let attestation = env::read::<Attestation>();
    let _ = pre_state.process_attestation(&attestation);
}
