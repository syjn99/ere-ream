use std::env;
use std::path::PathBuf;

use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType};

pub fn new_sp1_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<EreSP1> {
    let guest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../guests/sp1");
    let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;

    Ok(EreSP1::new(program, prover_resource))
}
