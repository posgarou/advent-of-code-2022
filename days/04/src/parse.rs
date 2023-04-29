use std::ops::RangeInclusive;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Error parsing range pair: {0}")]
    RangePairError(String),
    #[error("Error parsing number: {0}")]
    NumberError(#[from] std::num::ParseIntError),
}

pub fn parse_range_pair(
    line: String,
) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>), ParseError> {
    let parts = line.split(',').collect::<Vec<&str>>();

    if parts.len() != 2 {
        return Err(ParseError::RangePairError(line));
    }

    let first_range_parts = parts[0].split('-').collect::<Vec<&str>>();
    let second_range_parts = parts[1].split('-').collect::<Vec<&str>>();

    if first_range_parts.len() != 2 {
        return Err(ParseError::RangePairError(line));
    }

    if second_range_parts.len() != 2 {
        return Err(ParseError::RangePairError(line));
    }

    let first_range_start = first_range_parts[0].parse::<u32>()?;
    let first_range_end = first_range_parts[1].parse::<u32>()?;

    let second_range_start = second_range_parts[0].parse::<u32>()?;
    let second_range_end = second_range_parts[1].parse::<u32>()?;

    let first_range = first_range_start..=first_range_end;
    let second_range = second_range_start..=second_range_end;

    Ok((first_range, second_range))
}
