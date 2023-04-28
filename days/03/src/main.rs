use day03::{read_and_prioritize_rucksacks, read_and_prioritize_rucksacks_by_group};

pub fn main() {
    match read_and_prioritize_rucksacks("input.txt") {
        Ok(priority) => println!("Total priority: {}", priority),
        Err(e) => println!("Error: {}", e),
    }

    match read_and_prioritize_rucksacks_by_group("input.txt") {
        Ok(priority) => println!("Total priority by group: {}", priority),
        Err(e) => println!("Error: {}", e),
    }
}
