use day02::read_and_parse_and_score::read_and_parse_and_score;

fn main() {
    let filename = "input.txt";

    match read_and_parse_and_score(filename) {
        Ok(score) => println!("Score: {}", score),
        Err(e) => println!("Error: {}", e),
    }
}
