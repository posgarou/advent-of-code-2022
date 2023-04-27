use crate::round_parser::{ParseRoundError, RoundActionParser, RoundParser};
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

pub fn read_and_parse_and_score<T>(
    filename: &str,
    parser: RoundParser<T>,
) -> Result<i32, ParseAndScoreRoundError>
where
    T: RoundActionParser,
{
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let round = parser.parse(line)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round_parser::ContextualRoundActionParser;

    #[test]
    fn test_default_read_and_parse_and_score() {
        let filename = "fixtures/rounds.txt";
        let parser = RoundParser::default();

        let score = read_and_parse_and_score(filename, parser).unwrap();

        assert_eq!(score, 15);
    }

    #[test]
    fn test_contextual_read_and_parse_and_score() {
        let filename = "fixtures/rounds.txt";
        let parser = RoundParser::new(ContextualRoundActionParser::new());

        let score = read_and_parse_and_score(filename, parser).unwrap();

        assert_eq!(score, 12);
    }
}
