use day04::count_overlapped_ranges;

pub fn main() {
    match count_overlapped_ranges("input.txt") {
        Ok(count) => println!("{} ranges overlap", count),
        Err(e) => println!("Error counting overlapped ranges: {}", e),
    }
}
