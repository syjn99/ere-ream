use std::env;
use std::path::PathBuf;

use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType};

pub fn new_risc0_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<EreRisc0> {
    let guest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../guests/risc0");
    let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;

    Ok(EreRisc0::new(program, prover_resource))
}
