use day03::read_and_prioritize_rucksacks;

pub fn main() {
    match read_and_prioritize_rucksacks("input.txt") {
        Ok(priority) => println!("Total priority: {}", priority),
        Err(e) => println!("Error: {}", e),
    }
}
