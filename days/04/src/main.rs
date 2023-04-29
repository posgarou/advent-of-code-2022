use day04::count_overlapped_ranges;

pub fn main() {
    match count_overlapped_ranges("input.txt") {
        Ok((contained_ranges_count, overlapped_ranges_count)) => {
            println!("{} ranges are contained", contained_ranges_count);
            println!("{} ranges overlap", overlapped_ranges_count);
        }
        Err(e) => println!("Error counting overlapped ranges: {}", e),
    }
}
