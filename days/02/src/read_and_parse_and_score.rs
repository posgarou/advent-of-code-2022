use crate::round::RoundAction;
use crate::round_parser::{ParseRoundError, RoundParser};
use crate::score_round::score_round;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseAndScoreRoundError {
    #[error("Error reading file")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing round")]
    ParseRoundError(#[from] ParseRoundError),
}

pub fn read_and_parse_and_score(filename: &str) -> Result<i32, ParseAndScoreRoundError> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let round = RoundParser::parse(line, parse_opponent_action, parse_user_action)?;
        total_score += score_round(&round);
    }

    Ok(total_score)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_user_action(s: &str) -> Result<RoundAction, ParseRoundError> {
    match s {
        "X" => Ok(RoundAction::Rock),
        "Y" => Ok(RoundAction::Paper),
        "Z" => Ok(RoundAction::Scissors),
        _ => Err(ParseRoundError::InvalidUserActionError(s.to_string())),
    }
}

fn parse_opponent_action(s: &str) -> Result<RoundAction, ParseRoundError> {
    match s {
        "A" => Ok(RoundAction::Rock),
        "B" => Ok(RoundAction::Paper),
        "C" => Ok(RoundAction::Scissors),
        _ => Err(ParseRoundError::InvalidOpponentActionError(s.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_parse_and_score() {
        let filename = "fixtures/rounds.txt";

        let score = read_and_parse_and_score(filename).unwrap();

        assert_eq!(score, 15);
    }
}
