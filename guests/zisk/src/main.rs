#![no_main]
ziskos::entrypoint!(main);

use ream_consensus::{attestation::Attestation, electra::beacon_state::BeaconState};

pub fn main() {
    let mut pre_state: BeaconState = bincode::deserialize(&ziskos::read_input()).unwrap();
    let attestation: Attestation = bincode::deserialize(&ziskos::read_input()).unwrap();
    let _ = pre_state.process_attestation(&attestation);
}
