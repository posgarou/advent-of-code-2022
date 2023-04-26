use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

use crate::elf::Elf;

#[derive(Debug, Error)]
pub enum ElfParserError {
    #[error("Error reading file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Error parsing integer: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

pub fn read_elves_from_file(filename: &str) -> Result<Vec<Elf>, ElfParserError> {
    let mut elves = vec![];
    let lines = read_lines(filename)?;

    let mut current_food_items: Vec<i32> = vec![];

    for line in lines.into_iter() {
        let line = line?;

        if line.is_empty() && !current_food_items.is_empty() {
            elves.push(Elf::from(current_food_items));
            current_food_items = vec![];
            continue;
        }

        let calories = line.parse()?;
        current_food_items.push(calories);
    }

    if !current_food_items.is_empty() {
        elves.push(Elf::from(current_food_items));
    }

    Ok(elves)
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
