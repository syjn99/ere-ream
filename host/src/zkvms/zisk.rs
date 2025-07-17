use std::env;
use std::path::PathBuf;

use ere_zisk::{EreZisk, RV64_IMA_ZISK_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType};

pub fn new_zisk_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<EreZisk> {
    let guest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../guests/zisk");
    let program = RV64_IMA_ZISK_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;

    Ok(EreZisk::new(program, prover_resource))
}
