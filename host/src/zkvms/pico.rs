use std::env;
use std::path::PathBuf;

use ere_pico::{ErePico, PICO_TARGET};
use zkvm_interface::{Compiler, ProverResourceType};

pub fn new_pico_zkvm(prover_resource: ProverResourceType) -> anyhow::Result<ErePico> {
    let guest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../guests/pico");
    let program = PICO_TARGET::compile(&guest_dir)?;

    Ok(ErePico::new(program, prover_resource))
}
