mod prioritizer;
mod rucksack;
mod rucksack_reader;

use prioritizer::{prioritize, PrioritizationError};
use rucksack_reader::{RucksackReader, RucksackReaderError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadAndPrioritizeError {
    #[error("Error reading file: {0}")]
    RucksackReaderError(#[from] RucksackReaderError),
    #[error("Error prioritizing rucksack pair: {0}")]
    PrioritizationError(#[from] PrioritizationError),
}

pub fn read_and_prioritize_rucksacks(filename: &str) -> Result<i32, ReadAndPrioritizeError> {
    let rucksack_pairs = RucksackReader::read_from_file(filename)?;
    let mut total_priority = 0;

    for pair in rucksack_pairs {
        let priority = prioritize(&pair)?;
        total_priority += priority;
    }

    Ok(total_priority)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_and_prioritize_rucksacks() {
        assert_eq!(
            read_and_prioritize_rucksacks("fixtures/rucksacks.txt").unwrap(),
            157
        );
    }
}
