use thiserror::Error;

/// A command to move crates from one stack to another
///
/// # Examples
///
/// ```
/// use day05::crate_stacks::MoveCommand;
///
/// let command = MoveCommand::new(1, 2, 3);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct MoveCommand {
    count: i32,
    from: i32,
    to: i32,
}

impl MoveCommand {
    pub fn new(count: i32, from: i32, to: i32) -> MoveCommand {
        MoveCommand { count, from, to }
    }
}

#[derive(Debug, Error)]
pub enum CrateMoveError {
    #[error("Invalid move command: {0}")]
    InvalidMoveCommand(String),
}

/// A set of crate stacks, represented as a vector of vectors of chars.
///
/// # Examples
///
/// ```
/// use day05::crate_stacks::CrateStacks;
///
/// let stacks = CrateStacks::new(vec![vec!['A', 'B'], vec!['C']]);
///
/// assert_eq!(stacks.stack_count(), 2);
/// ```
///
/// ```
/// use day05::crate_stacks::{CrateStacks, MoveCommand};
///
/// let mut stacks = CrateStacks::new(vec![vec!['A', 'B'], vec!['C']]);
///
/// stacks.try_move(MoveCommand::new(1, 1, 2)).unwrap();
///
/// assert_eq!(stacks, CrateStacks::new(vec![vec!['A'], vec!['C', 'B']]));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    pub fn new(stacks: Vec<Vec<char>>) -> CrateStacks {
        CrateStacks { stacks }
    }

    /// Tries to execute a list of move commands
    ///
    /// # Arguments
    ///
    /// * `commands` - The list of commands to execute
    ///
    /// # Returns
    ///
    /// If all commands are executed successfully, returns Ok(()). Otherwise, returns an error.
    ///
    /// # Example
    ///
    /// ```
    /// use day05::crate_stacks::{CrateStacks, MoveCommand};
    ///
    /// let mut stacks = CrateStacks::new(vec![vec!['A', 'B'], vec!['C']]);
    ///
    /// stacks.try_moves(vec![
    ///    MoveCommand::new(1, 1, 2),
    ///   MoveCommand::new(1, 2, 1),
    /// ]).unwrap();
    ///
    /// assert_eq!(stacks, CrateStacks::new(vec![vec!['A', 'B'], vec!['C']]));
    /// ```
    pub fn try_moves(&mut self, commands: Vec<MoveCommand>) -> Result<(), CrateMoveError> {
        for command in commands {
            self.try_move(command)?;
        }

        Ok(())
    }

    pub fn try_move(&mut self, command: MoveCommand) -> Result<(), CrateMoveError> {
        let MoveCommand { count, from, to } = command;

        println!("Moving {} crates from {} to {}", count, from, to);

        if count < 0 {
            return Err(CrateMoveError::InvalidMoveCommand(format!(
                "Cannot move negative number of crates: {}",
                count
            )));
        }

        if from == to {
            return Err(CrateMoveError::InvalidMoveCommand(format!(
                "Cannot move crates from and to the same stack: {}",
                from
            )));
        }

        let valid_indexes = 1..=self.stack_count() as i32;

        if !valid_indexes.contains(&from) {
            return Err(CrateMoveError::InvalidMoveCommand(format!(
                "Invalid from index: {}",
                from
            )));
        }

        if !valid_indexes.contains(&to) {
            return Err(CrateMoveError::InvalidMoveCommand(format!(
                "Invalid to index: {}",
                to
            )));
        }

        let from_stack = &mut self.stacks[from as usize - 1];

        if from_stack.len() < count as usize {
            return Err(CrateMoveError::InvalidMoveCommand(format!(
                "Cannot move {} crates from stack {} because it only has {} crates",
                count,
                from,
                from_stack.len()
            )));
        }

        let mut crates_to_move: Vec<char> = from_stack
            .drain(from_stack.len() - count as usize..)
            .rev()
            .collect();

        let to_stack = &mut self.stacks[to as usize - 1];

        to_stack.append(&mut crates_to_move);

        Ok(())
    }

    pub fn get_top_crates(&self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .cloned()
            .collect()
    }

    pub fn stack_count(&self) -> usize {
        self.stacks.len()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let stacks = vec![vec!['A', 'B', 'C'], vec![], vec![]];
        let mut crate_stacks = CrateStacks { stacks: stacks };

        crate_stacks.try_move(MoveCommand::new(2, 1, 2)).unwrap();

        assert_eq!(crate_stacks.stacks[0], vec!['A']);
        assert_eq!(crate_stacks.stacks[1], vec!['C', 'B']);
        assert_eq!(crate_stacks.stacks[2].len(), 0);
    }

    #[test]
    fn test_get_top_crates() {
        let stacks = vec![vec!['A', 'B', 'C'], vec![], vec!['D']];
        let crate_stacks = CrateStacks { stacks: stacks };

        assert_eq!(crate_stacks.get_top_crates(), vec!['C', ' ', 'D']);
    }
}
