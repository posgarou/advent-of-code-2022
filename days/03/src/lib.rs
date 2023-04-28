mod prioritizer;
mod rucksack;
mod rucksack_reader;

use prioritizer::{get_item_priority, PrioritizationError};
use rucksack::RucksackGetCommonItemError;
use rucksack_reader::{RucksackReader, RucksackReaderError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadAndPrioritizeError {
    #[error("Error reading file: {0}")]
    RucksackReaderError(#[from] RucksackReaderError),
    #[error("Error getting repeated char: {0}")]
    RucksackGetRepeatedCharError(#[from] RucksackGetCommonItemError),
    #[error("Error prioritizing rucksack pair: {0}")]
    PrioritizationError(#[from] PrioritizationError),
}

pub fn read_and_prioritize_rucksacks(filename: &str) -> Result<i32, ReadAndPrioritizeError> {
    let rucksack_pairs = RucksackReader::read_from_file(filename)?;
    let mut total_priority = 0;

    for pair in rucksack_pairs {
        let repeated_char = pair.get_common_item()?;
        let priority = get_item_priority(&repeated_char)?;
        total_priority += priority;
    }

    Ok(total_priority)
}

pub fn read_and_prioritize_rucksacks_by_group(
    filename: &str,
) -> Result<i32, ReadAndPrioritizeError> {
    let rucksack_pairs = RucksackReader::read_from_file(filename)?;
    let mut total_priority = 0;

    for pairs in rucksack_pairs.chunks(3) {
        let first_pair = &pairs[0];
        let second_pair = &pairs[1];
        let third_pair = &pairs[2];

        let repeated_char = first_pair.get_common_item_with((second_pair, third_pair))?;

        let priority = get_item_priority(&repeated_char)?;
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
