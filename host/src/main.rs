use zkvm_interface::{Input, ProverResourceType, zkVM};

mod zkvms;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zkvm = zkvms::new_zkvm(ProverResourceType::Cpu)?;

    let mut input = Input::new();
    input.write(10);

    let report = zkvm.execute(&input)?;

    // Print the execution report
    println!("Total cycles: {}", report.total_num_cycles);
    for (region, cycles) in report.region_cycles.iter() {
        println!("  Region: {}, Cycles: {}", region, cycles);
    }
    println!("Execution duration: {:?}", report.execution_duration);

    Ok(())
}
