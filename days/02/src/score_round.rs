use crate::round::{Round, RoundAction, RoundOutcome};

pub fn score_round(round: &Round) -> i32 {
    let action_score = match round.user_action {
        RoundAction::Rock => 1,
        RoundAction::Paper => 2,
        RoundAction::Scissors => 3,
    };

    let outcome_score = match round.outcome() {
        RoundOutcome::Win => 6,
        RoundOutcome::Loss => 0,
        RoundOutcome::Draw => 3,
    };

    action_score + outcome_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round_win() {
        let round = Round::new(RoundAction::Rock, RoundAction::Scissors);
        assert_eq!(score_round(&round), 7);

        let round = Round::new(RoundAction::Paper, RoundAction::Rock);
        assert_eq!(score_round(&round), 8);

        let round = Round::new(RoundAction::Scissors, RoundAction::Paper);
        assert_eq!(score_round(&round), 9);
    }

    #[test]
    fn test_score_round_loss() {
        let round = Round::new(RoundAction::Rock, RoundAction::Paper);
        assert_eq!(score_round(&round), 1);

        let round = Round::new(RoundAction::Paper, RoundAction::Scissors);
        assert_eq!(score_round(&round), 2);

        let round = Round::new(RoundAction::Scissors, RoundAction::Rock);
        assert_eq!(score_round(&round), 3);
    }

    #[test]
    fn test_score_round_draw() {
        let round = Round::new(RoundAction::Rock, RoundAction::Rock);
        assert_eq!(score_round(&round), 4);

        let round = Round::new(RoundAction::Paper, RoundAction::Paper);
        assert_eq!(score_round(&round), 5);

        let round = Round::new(RoundAction::Scissors, RoundAction::Scissors);
        assert_eq!(score_round(&round), 6);
    }
}
