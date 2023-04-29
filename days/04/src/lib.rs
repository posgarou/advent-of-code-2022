pub mod parse;
pub mod range_pair;
pub mod util;

use crate::parse::parse_range_pair;
use crate::range_pair::{contains, intersects};
use crate::util::read_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CountError {
    #[error("Error reading lines from file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing range pair: {0}")]
    ParseError(#[from] crate::parse::ParseError),
}

pub fn count_overlapped_ranges(filename: &str) -> Result<(i32, i32), CountError> {
    let mut contained_ranges_count = 0;
    let mut intersected_ranges_count = 0;

    for line in read_lines(filename)? {
        let (first_range, second_range) = parse_range_pair(line?)?;

        if contains(&first_range, &second_range) || contains(&second_range, &first_range) {
            contained_ranges_count += 1;
        }

        if intersects(&first_range, &second_range) {
            intersected_ranges_count += 1;
        }
    }

    Ok((contained_ranges_count, intersected_ranges_count))
}
