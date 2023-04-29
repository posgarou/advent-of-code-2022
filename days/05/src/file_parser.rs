use crate::crate_stacks::{CrateStacks, MoveCommand};
use crate::util::{read_lines, transpose};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, Lines};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCrateStacksError {
    #[error("File read error: {0}")]
    FileReadError(#[from] std::io::Error),
}

/// Parse a file containing crate stacks and move commands
///
/// # Arguments
///
/// * `filename` - The name of the file to parse
///
/// # Returns
///
/// A tuple containing the crate stacks and the move commands
///
/// # Example
///
/// ```
/// use day05::file_parser::parse_file;
///
/// let (crate_stacks, move_commands) = parse_file("fixtures/fixture.txt").unwrap();
///
/// assert_eq!(crate_stacks.stack_count(), 3);
/// assert_eq!(move_commands.len(), 4);
/// ```
///
/// # Errors
///
/// If the file cannot be read, an error is returned
pub fn parse_file(
    filename: &str,
) -> Result<(CrateStacks, Vec<MoveCommand>), ParseCrateStacksError> {
    let mut lines = read_lines(filename)?;

    let crate_stacks = read_crate_stacks_from_file(&mut lines)?;
    let move_commands = read_move_rows_from_file(&mut lines)?;

    Ok((crate_stacks, move_commands))
}

fn read_crate_stacks_from_file<B: BufRead>(
    lines: &mut Lines<B>,
) -> Result<CrateStacks, std::io::Error> {
    let mut rows = vec![];

    for line in lines {
        let line = line?;

        if line.contains('[') {
            let row = parse_crate_row(&line);

            rows.push(row);
        } else {
            break;
        }
    }

    // Transpose the rows into columns
    // Remove the None values from each column
    let columns: Vec<Vec<char>> = transpose(rows)
        .iter()
        .map(|column| column.iter().rev().filter_map(|c| *c).collect())
        .collect();

    Ok(CrateStacks::new(columns))
}

fn read_move_rows_from_file<B: BufRead>(
    lines: &mut Lines<B>,
) -> Result<Vec<MoveCommand>, std::io::Error> {
    let mut moves = vec![];

    for line in lines {
        let line = line?;

        let command = parse_move_row(&line);
        if let Some(command) = command {
            moves.push(command);
        }
    }

    Ok(moves)
}

fn parse_crate_row(line: &str) -> Vec<Option<char>> {
    let mut crates = vec![];

    for column in line.chars().skip(1).step_by(4) {
        match column {
            ' ' => crates.push(None),
            _ => crates.push(Some(column)),
        }
    }

    crates
}

fn parse_move_row(line: &str) -> Option<MoveCommand> {
    lazy_static! {
        static ref MOVELINE_REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }

    let captures = MOVELINE_REGEX.captures(line)?;

    match (captures.get(1), captures.get(2), captures.get(3)) {
        (Some(count), Some(from), Some(to)) => {
            let count = count.as_str().parse::<i32>().ok()?;
            let from = from.as_str().parse::<i32>().ok()?;
            let to = to.as_str().parse::<i32>().ok()?;

            Some(MoveCommand::new(count, from, to))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixture() {
        let (crate_stacks, moves) = parse_file("fixtures/fixture.txt").unwrap();

        assert_eq!(
            crate_stacks,
            CrateStacks::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
        );
        assert_eq!(
            moves,
            vec![
                MoveCommand::new(1, 2, 1),
                MoveCommand::new(3, 1, 3),
                MoveCommand::new(2, 2, 1),
                MoveCommand::new(1, 1, 2)
            ]
        );
    }

    #[test]
    fn test_parse_crate_row() {
        let line = "[A] [B] [C]";

        let crates = parse_crate_row(line);

        assert_eq!(crates, vec![Some('A'), Some('B'), Some('C')]);
    }

    #[test]
    fn test_parse_crate_row_with_empty_crates() {
        let line = "[A]     [C]";

        let crates = parse_crate_row(line);

        assert_eq!(crates, vec![Some('A'), None, Some('C')]);
    }

    #[test]
    fn test_parse_move_row() {
        let line = "move 1 from 2 to 3";

        let command = parse_move_row(line);

        assert_eq!(command, Some(MoveCommand::new(1, 2, 3)));
    }
}
