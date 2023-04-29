use day05::{crate_stacks::MoverStrategy, read_and_apply_moves};

pub fn main() {
    match read_and_apply_moves("input.txt", MoverStrategy::OneAtATime) {
        Ok(result) => println!("CrateMover 9000 Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match read_and_apply_moves("input.txt", MoverStrategy::AllAtOnce) {
        Ok(result) => println!("CrateMover 9001 Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
