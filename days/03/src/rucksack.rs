use thiserror::Error;

#[derive(Debug, Error)]
pub enum RucksackPairParserError {
    #[error("Invalid string length: {0}")]
    InvalidLengthError(usize),
}

#[derive(Debug, Error)]
pub enum RucksackGetCommonItemError {
    #[error("No repeated item found")]
    NoRepeatedItemError(),
    #[error("Multiple repeated items found: {0:?}")]
    MultipleRepeatedCharError(Vec<char>),
}

pub struct RucksackPair {
    rucksacks: (Rucksack, Rucksack),
}

impl RucksackPair {
    pub fn get_common_item(&self) -> Result<char, RucksackGetCommonItemError> {
        let mut repeated_chars = Vec::new();

        for item in self.rucksacks.0.get_items() {
            if self.rucksacks.1.get_items().contains(item) {
                if repeated_chars.contains(item) {
                    continue;
                }

                repeated_chars.push(*item);
            }
        }

        match repeated_chars.len() {
            0 => Err(RucksackGetCommonItemError::NoRepeatedItemError()),
            1 => Ok(repeated_chars[0]),
            _ => Err(RucksackGetCommonItemError::MultipleRepeatedCharError(
                repeated_chars,
            )),
        }
    }

    #[cfg(test)]
    fn get_rucksacks(&self) -> &(Rucksack, Rucksack) {
        &self.rucksacks
    }
}

impl TryFrom<String> for RucksackPair {
    type Error = RucksackPairParserError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let chars: Vec<char> = s.chars().collect();
        let char_count = chars.len();

        if chars.len() % 2 != 0 {
            return Err(RucksackPairParserError::InvalidLengthError(char_count));
        }

        let first_group = chars[0..char_count / 2].to_vec();
        let second_group = chars[char_count / 2..char_count].to_vec();

        Ok(RucksackPair {
            rucksacks: (Rucksack::new(first_group), Rucksack::new(second_group)),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rucksack {
    items: Vec<char>,
}

impl Rucksack {
    pub fn new(items: Vec<char>) -> Rucksack {
        Rucksack { items }
    }

    pub fn get_items(&self) -> &Vec<char> {
        &self.items
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_rucksack_pair_parser() {
        let rucksack_pair = RucksackPair::try_from(String::from("abcd")).unwrap();

        assert_eq!(
            rucksack_pair.get_rucksacks(),
            &(Rucksack::new(vec!['a', 'b']), Rucksack::new(vec!['c', 'd']))
        );
    }

    #[test]
    #[should_panic(expected = "InvalidLengthError(5)")]
    fn test_rucksack_pair_parser_invalid_length() {
        RucksackPair::try_from(String::from("abcde")).unwrap();
    }

    #[test]
    fn test_rucksack_pair_get_repeated_char() {
        let rucksack_pair = RucksackPair::try_from(String::from("abccde")).unwrap();

        assert_eq!(rucksack_pair.get_common_item().unwrap(), 'c');
    }
}
