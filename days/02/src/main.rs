use day02::read_and_parse_and_score::read_and_parse_and_score;
use day02::round_parser::*;

fn main() {
    let filename = "input.txt";

    let default_parser = RoundParser::default();
    let contextual_parser = RoundParser::new(ContextualRoundActionParser::new());

    match read_and_parse_and_score(filename, default_parser) {
        Ok(score) => println!("Part 1, Score: {}", score),
        Err(e) => println!("Error: {}", e),
    }

    match read_and_parse_and_score(filename, contextual_parser) {
        Ok(score) => println!("Part 2, Score: {}", score),
        Err(e) => println!("Error: {}", e),
    }
}
