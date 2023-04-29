use day05::{crate_stacks::MoverStrategy, read_and_apply_moves};

pub fn main() {
    match read_and_apply_moves("input.txt", MoverStrategy::AllAtOnce) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
