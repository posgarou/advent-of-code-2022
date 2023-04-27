use crate::round::{Round, RoundAction};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
}

#[derive(Debug, Error)]
pub enum ParseRoundError {
    #[error("Error parsing round")]
    InvalidFormatError(),
    #[error("Error parsing opponent action")]
    InvalidOpponentActionError(String),
    #[error("Error parsing user action")]
    InvalidUserActionError(String),
}

pub struct RoundParser {}

impl RoundParser {
    pub fn parse(
        line: String,
        opponent_action_parser: impl Fn(&str) -> Result<RoundAction, ParseRoundError>,
        user_action_parser: impl Fn(&str) -> Result<RoundAction, ParseRoundError>,
    ) -> Result<Round, ParseRoundError> {
        let matches = LINE_REGEX
            .captures(&line)
            .ok_or(ParseRoundError::InvalidFormatError())?;
        let (opponent_action, user_action) = (matches[1].to_string(), matches[2].to_string());

        let opponent_action = opponent_action_parser(&opponent_action)?;
        let user_action = user_action_parser(&user_action)?;

        Ok(Round::new(user_action, opponent_action))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "A X".to_string();

        let round =
            RoundParser::parse(line, |_s| Ok(RoundAction::Rock), |_s| Ok(RoundAction::Rock))
                .unwrap();
        assert_eq!(round.opponent_action, RoundAction::Rock);
        assert_eq!(round.user_action, RoundAction::Rock);
    }

    #[test]
    fn test_parse_invalid_format() {
        let line = "A".to_string();
        let round =
            RoundParser::parse(line, |_s| Ok(RoundAction::Rock), |_s| Ok(RoundAction::Rock));
        assert!(round.is_err());
    }

    #[test]
    fn test_parse_invalid_opponent_action() {
        let line = "D X".to_string();
        let round =
            RoundParser::parse(line, |_s| Ok(RoundAction::Rock), |_s| Ok(RoundAction::Rock));
        assert!(round.is_err());
    }

    #[test]
    fn test_parse_invalid_user_action() {
        let line = "A D".to_string();
        let round =
            RoundParser::parse(line, |_s| Ok(RoundAction::Rock), |_s| Ok(RoundAction::Rock));
        assert!(round.is_err());
    }
}
