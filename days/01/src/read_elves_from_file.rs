use std::fs::File;
use std::io::{self, BufRead};
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

pub fn read_elves_from_file(filename: &str) -> Result<Vec<Elf>, std::io::Error> {
    let mut elves = vec![];
    let mut lines = read_lines(filename)?;

    loop {
        let foods = lines
            .by_ref()
            .take_while(|line| !line.as_ref().unwrap().is_empty())
            .map(|line| Food::new(line.unwrap().parse().unwrap()))
            .collect::<Vec<Food>>();

        if foods.is_empty() {
            break;
        }

        let elf = Elf::from(foods);
        elves.push(elf);
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