use crate::round::{get_user_action_by_outcome, Round, RoundAction, RoundOutcome};
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

pub struct RoundParser<T: RoundActionParser> {
    action_parser: T,
}

impl<T: RoundActionParser> RoundParser<T> {
    pub fn new(action_parser: T) -> Self
    where
        T: RoundActionParser,
    {
        RoundParser { action_parser }
    }

    pub fn parse(&self, line: String) -> Result<Round, ParseRoundError> {
        let matches = LINE_REGEX
            .captures(&line)
            .ok_or(ParseRoundError::InvalidFormatError())?;

        let (opponent_action_code, user_action_code) =
            (matches[1].to_string(), matches[2].to_string());

        let opponent_action = self
            .action_parser
            .parse_opponent_action(&opponent_action_code)?;

        let user_action = self
            .action_parser
            .parse_user_action(&user_action_code, opponent_action)?;

        Ok(Round::new(user_action, opponent_action))
    }
}

impl Default for RoundParser<DefaultRoundActionParser> {
    fn default() -> Self {
        Self::new(DefaultRoundActionParser::new())
    }
}

pub trait RoundActionParser {
    fn parse_opponent_action(&self, s: &str) -> Result<RoundAction, ParseRoundError>;
    fn parse_user_action(
        &self,
        s: &str,
        opponent_action: RoundAction,
    ) -> Result<RoundAction, ParseRoundError>;
}
pub struct DefaultRoundActionParser {}

impl DefaultRoundActionParser {
    pub fn new() -> Self {
        DefaultRoundActionParser {}
    }
}

impl Default for DefaultRoundActionParser {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundActionParser for DefaultRoundActionParser {
    fn parse_opponent_action(&self, s: &str) -> Result<RoundAction, ParseRoundError> {
        match s {
            "A" => Ok(RoundAction::Rock),
            "B" => Ok(RoundAction::Paper),
            "C" => Ok(RoundAction::Scissors),
            _ => Err(ParseRoundError::InvalidOpponentActionError(s.to_string())),
        }
    }

    fn parse_user_action(
        &self,
        s: &str,
        _opponent_action: RoundAction,
    ) -> Result<RoundAction, ParseRoundError> {
        match s {
            "X" => Ok(RoundAction::Rock),
            "Y" => Ok(RoundAction::Paper),
            "Z" => Ok(RoundAction::Scissors),
            _ => Err(ParseRoundError::InvalidUserActionError(s.to_string())),
        }
    }
}

pub struct ContextualRoundActionParser {
    default_parser: DefaultRoundActionParser,
}

impl ContextualRoundActionParser {
    pub fn new() -> Self {
        ContextualRoundActionParser {
            default_parser: DefaultRoundActionParser {},
        }
    }
}

impl Default for ContextualRoundActionParser {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundActionParser for ContextualRoundActionParser {
    fn parse_opponent_action(&self, s: &str) -> Result<RoundAction, ParseRoundError> {
        self.default_parser.parse_opponent_action(s)
    }

    fn parse_user_action(
        &self,
        s: &str,
        opponent_action: RoundAction,
    ) -> Result<RoundAction, ParseRoundError> {
        match s {
            "X" => Ok(get_user_action_by_outcome(
                opponent_action,
                RoundOutcome::Loss,
            )),
            "Y" => Ok(get_user_action_by_outcome(
                opponent_action,
                RoundOutcome::Draw,
            )),
            "Z" => Ok(get_user_action_by_outcome(
                opponent_action,
                RoundOutcome::Win,
            )),
            _ => Err(ParseRoundError::InvalidUserActionError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_parse() {
        let line = "A X".to_string();
        let parser = RoundParser::new(DefaultRoundActionParser::new());

        let round = parser.parse(line).unwrap();

        assert_eq!(round.opponent_action, RoundAction::Rock);
        assert_eq!(round.user_action, RoundAction::Rock);
    }

    #[test]
    fn test_default_parse_invalid_format() {
        let line = "A".to_string();
        let parser = RoundParser::new(DefaultRoundActionParser::new());

        let result = parser.parse(line);

        assert!(result.is_err());
    }

    #[test]
    fn test_default_parse_invalid_opponent_action() {
        let line = "D X".to_string();
        let parser = RoundParser::new(DefaultRoundActionParser::new());

        let result = parser.parse(line);

        assert!(result.is_err());
    }

    #[test]
    fn test_default_parse_invalid_user_action() {
        let line = "A D".to_string();
        let parser = RoundParser::new(DefaultRoundActionParser::new());

        let result = parser.parse(line);

        assert!(result.is_err());
    }

    #[test]
    fn test_contextual_parse_win() {
        let line = "A Z".to_string();
        let parser = RoundParser::new(ContextualRoundActionParser::new());

        let round = parser.parse(line).unwrap();

        assert_eq!(round.opponent_action, RoundAction::Rock);
        assert_eq!(round.user_action, RoundAction::Paper);
    }

    #[test]
    fn test_contextual_parse_loss() {
        let line = "A X".to_string();
        let parser = RoundParser::new(ContextualRoundActionParser::new());

        let round = parser.parse(line).unwrap();

        assert_eq!(round.opponent_action, RoundAction::Rock);
        assert_eq!(round.user_action, RoundAction::Scissors);
    }

    #[test]
    fn test_contextual_parse_draw() {
        let line = "A Y".to_string();
        let parser = RoundParser::new(ContextualRoundActionParser::new());

        let round = parser.parse(line).unwrap();

        assert_eq!(round.opponent_action, RoundAction::Rock);
        assert_eq!(round.user_action, RoundAction::Rock);
    }
}
