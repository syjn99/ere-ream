use risc0_zkvm::guest::env;

fn main() {
    let input: u32 = env::read();
    let dummy = input * 2;
}
