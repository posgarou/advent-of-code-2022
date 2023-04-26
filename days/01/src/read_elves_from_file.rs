use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;
use thiserror::Error;

use crate::elf::Elf;
use crate::food::Food;

#[derive(Debug, Error)]
pub enum ElfParserError {
    #[error("Error reading file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Error parsing integer: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

pub fn read_elves_from_file(filename: &str) -> Result<Vec<Elf>, ElfParserError> {
    let mut elves = vec![];
    let mut lines = read_lines(filename)?;

    while let Some(elf) = read_elf_from_lines(&mut lines)? {
        elves.push(elf);
    }

    Ok(elves)
}

fn read_elf_from_lines<B: BufRead>(lines: &mut Lines<B>) -> Result<Option<Elf>, ElfParserError> {
    let mut food_items = vec![];

    for line in lines {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let food_item: Food = line.try_into()?;
        food_items.push(food_item);
    }

    if food_items.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(Elf::from(food_items)))
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_elves_from_file() {
        let fixture = "fixtures/elves.txt";
        let elves = read_elves_from_file(fixture).unwrap();
        assert_eq!(
            elves,
            vec![
                Elf::from(vec![1000, 2000, 3000]),
                Elf::from(vec![4000]),
                Elf::from(vec![5000, 6000]),
                Elf::from(vec![7000, 8000, 9000]),
                Elf::from(vec![10000]),
            ]
        );
    }

    #[test]
    fn test_read_lines() {
        let fixture = "fixtures/elves.txt";
        let lines = read_lines(fixture).unwrap();
        let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
        assert_eq!(
            lines,
            vec![
                "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000",
                "", "10000"
            ]
        );
    }
}
