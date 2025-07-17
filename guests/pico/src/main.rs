#![no_main]
pico_sdk::entrypoint!(main);

use pico_sdk::io::read_as;
use ream_consensus::{attestation::Attestation, electra::beacon_state::BeaconState};

pub fn main() {
    let mut pre_state = read_as::<BeaconState>();
    let attestation = read_as::<Attestation>();
    let _ = pre_state.process_attestation(&attestation);
}
