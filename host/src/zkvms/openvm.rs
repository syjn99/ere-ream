use std::env;
use std::path::PathBuf;

use ere_openvm::{EreOpenVM, OPENVM_TARGET};
use zkvm_interface::{Compiler, ProverResourceType};

pub fn new_openvm_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<EreOpenVM> {
    let guest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../guests/openvm");
    let program = OPENVM_TARGET::compile(&PathBuf::from(guest_dir))?;

    Ok(EreOpenVM::new(program, prover_resource))
}
