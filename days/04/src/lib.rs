pub mod parse;
pub mod util;

use std::ops::RangeInclusive;

use crate::parse::parse_range_pair;
use crate::util::read_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CountError {
    #[error("Error reading lines from file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing range pair: {0}")]
    ParseError(#[from] crate::parse::ParseError),
}

pub fn count_overlapped_ranges(filename: &str) -> Result<i32, CountError> {
    let mut count = 0;

    for line in read_lines(filename)? {
        let (first_range, second_range) = parse_range_pair(line?)?;

        if contains(&first_range, &second_range) || contains(&second_range, &first_range) {
            count += 1;
        }
    }

    Ok(count)
}

fn contains(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> bool {
    let (self_start, self_end) = (left.start(), left.end());
    let (other_start, other_end) = (right.start(), right.end());

    if self_start <= other_start && self_end >= other_end {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(contains(&(1..=3), &(1..=2)));
        assert!(contains(&(1..=3), &(2..=2)));
        assert!(contains(&(1..=3), &(2..=3)));
        assert!(!contains(&(1..=3), &(2..=4)));
        assert!(!contains(&(2..=4), &(1..=3)));
        assert!(!contains(&(1..=3), &(4..=6)));
        assert!(!contains(&(4..=6), &(1..=3)));
    }

    #[test]
    fn test_count_overlapped_ranges() {
        assert_eq!(count_overlapped_ranges("fixtures/fixture.txt").unwrap(), 2);
    }
}
