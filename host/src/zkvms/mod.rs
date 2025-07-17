use zkvm_interface::ProverResourceType;

#[cfg(all(
    not(feature = "risc0"),
    not(feature = "sp1"),
    not(feature = "openvm"),
    not(feature = "pico")
))]
pub fn new_zkvm(
    _prover_resource: ProverResourceType,
) -> anyhow::Result<Box<dyn zkvm_interface::zkVM>> {
    Err::<Box<dyn zkvm_interface::zkVM>, anyhow::Error>(anyhow::anyhow!(
        "No zkVM feature enabled. Please enable either 'risc0' or 'sp1'."
    ))
}

#[cfg(feature = "sp1")]
pub mod sp1;

#[cfg(feature = "sp1")]
pub fn new_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<impl zkvm_interface::zkVM> {
    sp1::new_sp1_zkvm(prover_resource)
}

#[cfg(feature = "risc0")]
pub mod risc0;

#[cfg(feature = "risc0")]
pub fn new_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<impl zkvm_interface::zkVM> {
    risc0::new_risc0_zkvm(prover_resource)
}

#[cfg(feature = "openvm")]
pub mod openvm;

#[cfg(feature = "openvm")]
pub fn new_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<impl zkvm_interface::zkVM> {
    openvm::new_openvm_zkvm(prover_resource)
}

#[cfg(feature = "pico")]
pub mod pico;

#[cfg(feature = "pico")]
pub fn new_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<impl zkvm_interface::zkVM> {
    pico::new_pico_zkvm(prover_resource)
}
