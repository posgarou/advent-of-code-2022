use crate::rucksack::{RucksackPair, RucksackPairParserError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RucksackReaderError {
    #[error("Error reading file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing rucksack pair: {0})")]
    RucksackPairParserError(#[from] RucksackPairParserError),
}

pub struct RucksackReader {}

impl RucksackReader {
    pub fn read_from_file(filename: &str) -> Result<Vec<RucksackPair>, RucksackReaderError> {
        let mut rucksack_pairs = Vec::new();

        for line in read_lines(filename)? {
            let line = line?;
            let pair = RucksackPair::try_from(line)?;

            rucksack_pairs.push(pair);
        }

        Ok(rucksack_pairs)
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
