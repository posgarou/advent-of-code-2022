#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundAction {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    Win,
    Loss,
    Draw,
}

// There are two other main approaches for implementing this logic.
//
// 1. Assign each action a number and use modulus arithmetic to determine the
//    answer.
// 2. Implement Ord for RoundAction
//
// Both of these approaches are more concise, but I think this approach is
// easier to understand because it's more explicit.

pub fn get_user_action_by_outcome(
    opponent_action: RoundAction,
    outcome: RoundOutcome,
) -> RoundAction {
    match (opponent_action, outcome) {
        (RoundAction::Rock, RoundOutcome::Win) => RoundAction::Paper,
        (RoundAction::Rock, RoundOutcome::Loss) => RoundAction::Scissors,
        (RoundAction::Paper, RoundOutcome::Win) => RoundAction::Scissors,
        (RoundAction::Paper, RoundOutcome::Loss) => RoundAction::Rock,
        (RoundAction::Scissors, RoundOutcome::Win) => RoundAction::Rock,
        (RoundAction::Scissors, RoundOutcome::Loss) => RoundAction::Paper,
        _ => opponent_action,
    }
}

pub fn get_opponent_action_by_outcome(
    user_action: RoundAction,
    outcome: RoundOutcome,
) -> RoundAction {
    match (user_action, outcome) {
        (RoundAction::Rock, RoundOutcome::Win) => RoundAction::Scissors,
        (RoundAction::Rock, RoundOutcome::Loss) => RoundAction::Paper,
        (RoundAction::Paper, RoundOutcome::Win) => RoundAction::Rock,
        (RoundAction::Paper, RoundOutcome::Loss) => RoundAction::Scissors,
        (RoundAction::Scissors, RoundOutcome::Win) => RoundAction::Paper,
        (RoundAction::Scissors, RoundOutcome::Loss) => RoundAction::Scissors,
        _ => user_action,
    }
}

#[derive(Debug)]
pub struct Round {
    pub opponent_action: RoundAction,
    pub user_action: RoundAction,
}

impl Round {
    pub fn new(user_action: RoundAction, opponent_action: RoundAction) -> Round {
        Round {
            opponent_action,
            user_action,
        }
    }

    pub fn outcome(&self) -> RoundOutcome {
        match (&self.user_action, &self.opponent_action) {
            (RoundAction::Rock, RoundAction::Scissors) => RoundOutcome::Win,
            (RoundAction::Paper, RoundAction::Rock) => RoundOutcome::Win,
            (RoundAction::Scissors, RoundAction::Paper) => RoundOutcome::Win,
            (RoundAction::Rock, RoundAction::Paper) => RoundOutcome::Loss,
            (RoundAction::Paper, RoundAction::Scissors) => RoundOutcome::Loss,
            (RoundAction::Scissors, RoundAction::Rock) => RoundOutcome::Loss,
            _ => RoundOutcome::Draw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcome_win() {
        let round1 = Round::new(RoundAction::Rock, RoundAction::Scissors);
        let round2 = Round::new(RoundAction::Paper, RoundAction::Rock);
        let round3 = Round::new(RoundAction::Scissors, RoundAction::Paper);

        assert_eq!(round1.outcome(), RoundOutcome::Win);
        assert_eq!(round2.outcome(), RoundOutcome::Win);
        assert_eq!(round3.outcome(), RoundOutcome::Win);
    }

    #[test]
    fn test_outcome_loss() {
        let round1 = Round::new(RoundAction::Rock, RoundAction::Paper);
        let round2 = Round::new(RoundAction::Paper, RoundAction::Scissors);
        let round3 = Round::new(RoundAction::Scissors, RoundAction::Rock);

        assert_eq!(round1.outcome(), RoundOutcome::Loss);
        assert_eq!(round2.outcome(), RoundOutcome::Loss);
        assert_eq!(round3.outcome(), RoundOutcome::Loss);
    }

    #[test]
    fn test_outcome_draw() {
        let round1 = Round::new(RoundAction::Rock, RoundAction::Rock);
        let round2 = Round::new(RoundAction::Paper, RoundAction::Paper);
        let round3 = Round::new(RoundAction::Scissors, RoundAction::Scissors);

        assert_eq!(round1.outcome(), RoundOutcome::Draw);
        assert_eq!(round2.outcome(), RoundOutcome::Draw);
        assert_eq!(round3.outcome(), RoundOutcome::Draw);
    }
}
