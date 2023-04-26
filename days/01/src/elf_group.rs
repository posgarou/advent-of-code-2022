use crate::elf::Elf;

pub struct ElfGroup<'a> {
    elves: Vec<&'a Elf>,
}

impl<'a> ElfGroup<'a> {
    pub fn new(elves: &mut Vec<&'a Elf>) -> ElfGroup<'a> {
        elves.sort_by_key(|elf| -elf.total_calories());

        ElfGroup {
            elves: elves.to_vec(),
        }
    }

    pub fn take_top_elves_by_total_calories(&self, count: usize) -> Vec<&'a Elf> {
        self.elves.iter().take(count).cloned().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_top_elves_by_total_calories() {
        let elves = vec![
            Elf::from(vec![1, 1, 1, 1, 1]),
            Elf::from(vec![2, 2, 2, 2, 2]),
            Elf::from(vec![3, 3, 3, 3, 3]),
            Elf::from(vec![4, 4, 4, 4, 4]),
            Elf::from(vec![5, 5, 5, 5, 5]),
        ];

        let mut elf_refs = elves.iter().collect();

        let elf_group = ElfGroup::new(&mut elf_refs);

        let top_elves = elf_group.take_top_elves_by_total_calories(3);

        assert_eq!(top_elves.len(), 3);
        assert_eq!(top_elves[0].total_calories(), 25);
        assert_eq!(top_elves[1].total_calories(), 20);
        assert_eq!(top_elves[2].total_calories(), 15);
    }
}
