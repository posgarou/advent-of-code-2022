use day01::elf_group::ElfGroup;
use day01::read_elves_from_file::read_elves_from_file;

fn main() {
    let fixture = "input.txt";

    match read_elves_from_file(fixture) {
        Ok(elves) => {
            let elf_group = ElfGroup::new(&mut elves.iter().collect());

            let top_three_elves = elf_group.take_top_elves_by_total_calories(3);

            match top_three_elves.get(0) {
                Some(elf) => println!("Elf with most calories has: {:?}", elf.total_calories()),
                None => println!("No elves found"),
            }

            let top_three_elf_calories = top_three_elves
                .iter()
                .map(|elf| elf.total_calories())
                .sum::<i32>();

            println!("Top three elves have: {:?}", top_three_elf_calories);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
