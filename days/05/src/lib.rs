use crate_stacks::MoverStrategy;

pub mod crate_stacks;
pub mod file_parser;

mod util;

pub fn read_and_apply_moves(
    filename: &str,
    strategy: MoverStrategy,
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut crate_stacks, move_commands) = file_parser::parse_file(filename)?;

    crate_stacks.try_moves(move_commands, strategy)?;

    Ok(crate_stacks.get_top_crates().iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_apply_moves() {
        let result =
            read_and_apply_moves("fixtures/fixture.txt", MoverStrategy::OneAtATime).unwrap();

        assert_eq!(result, "CMZ");
    }
}
