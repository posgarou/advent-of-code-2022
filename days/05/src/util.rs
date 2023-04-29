use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn transpose<T: Copy>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_count = rows.len();

    if row_count == 0 {
        return rows;
    }

    let col_count = rows[0].len();

    if col_count == 0 {
        return rows;
    }

    (0..col_count)
        .map(|column| (0..row_count).map(|row| rows[row][column]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

        assert_eq!(transpose(rows), expected);
    }
}
