#![no_main]
sp1_zkvm::entrypoint!(main);

fn main() {
    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: execute");
    let dummy = input * 2;
    println!("cycle-tracker-report-end: execute");
}
