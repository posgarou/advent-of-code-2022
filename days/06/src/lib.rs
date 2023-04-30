use std::{collections::HashSet, hash::Hash};

pub fn find_packet_start_in_file(filename: &str) -> Result<Option<usize>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;

    Ok(find_packet_start(&input))
}

fn find_packet_start(input: &str) -> Option<usize> {
    let window_size = 4;
    let chars = input.chars().collect::<Vec<char>>();

    chars
        .windows(window_size)
        .position(|window| dedupe(window).len() == window_size)
        .map(|index| index + window_size)
}

fn dedupe<T: Copy + Eq + Hash>(input: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut output = Vec::new();

    for item in input {
        if !seen.contains(item) {
            seen.insert(*item);
            output.push(*item);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dedupe() {
        let input = vec!['a', 'b', 'b', 'c', 'a', 'd'];

        assert_eq!(dedupe(&input), vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    pub fn test_find_packet_start() {
        assert_eq!(find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(find_packet_start("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(
            find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
        assert_eq!(find_packet_start("abc"), None);
        assert_eq!(find_packet_start("abcabc"), None);
        assert_eq!(find_packet_start(""), None);
    }
}
