use day05::read_and_apply_moves;

pub fn main() {
    match read_and_apply_moves("input.txt") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
