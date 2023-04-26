use day01::read_elves_from_file::read_elves_from_file;

fn main() {
    let fixture = "input.txt";

    match read_elves_from_file(fixture) {
        Ok(elves) => {
            let elf_with_most_calories = elves.iter().max_by_key(|elf| elf.total_calories());

            match elf_with_most_calories {
                Some(elf) => println!("Elf with most calories has: {:?}", elf.total_calories()),
                None => println!("No elves found"),
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
