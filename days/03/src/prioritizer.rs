use crate::rucksack::RucksackGetCommonItemError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrioritizationError {
    #[error("Error getting repeated char: {0}")]
    RucksackGetRepeatedCharError(#[from] RucksackGetCommonItemError),
    #[error("Error getting item priority: {0}")]
    GetItemPriorityError(char),
}

pub fn get_item_priority(item: &char) -> Result<i32, PrioritizationError> {
    let priorities: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    priorities
        .iter()
        .position(|&c| c == *item)
        .map(|i| i as i32 + 1)
        .ok_or(PrioritizationError::GetItemPriorityError(*item))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_get_item_priority() {
        assert_eq!(get_item_priority(&'a').unwrap(), 1);
        assert_eq!(get_item_priority(&'z').unwrap(), 26);
        assert_eq!(get_item_priority(&'A').unwrap(), 27);
        assert_eq!(get_item_priority(&'Z').unwrap(), 52);
    }
}
