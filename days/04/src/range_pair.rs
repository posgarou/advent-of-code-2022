use std::ops::RangeInclusive;

pub fn intersects<T>(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end())
}

pub fn contains<T>(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    left.contains(right.start()) && left.contains(right.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersets() {
        assert!(intersects(&(1..=3), &(1..=2)));
        assert!(intersects(&(1..=3), &(2..=2)));
        assert!(intersects(&(1..=3), &(2..=3)));
        assert!(intersects(&(1..=3), &(2..=4)));
        assert!(intersects(&(2..=4), &(1..=3)));

        assert!(!intersects(&(1..=3), &(4..=6)));
        assert!(!intersects(&(4..=6), &(1..=3)));
    }

    #[test]
    fn test_contains() {
        assert!(contains(&(1..=3), &(1..=2)));
        assert!(contains(&(1..=3), &(2..=2)));
        assert!(contains(&(1..=3), &(2..=3)));
        assert!(!contains(&(1..=3), &(2..=4)));
        assert!(!contains(&(2..=4), &(1..=3)));
        assert!(!contains(&(1..=3), &(4..=6)));
        assert!(!contains(&(4..=6), &(1..=3)));
    }
}
