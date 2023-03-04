#![allow(dead_code)]
fn index(nums: &[u64], n: usize) -> Option<u64> {
    Some(nums.get(n)?.pow(n as u32))
}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn sample_tests() {
        assert_eq!(
            index(&[1, 2, 3, 4], 2),
            Some(9),
            "Failed on the first sample test"
        );
        assert_eq!(
            index(&[1, 3, 10, 100], 3),
            Some(1000000),
            "Failed on the second sample test"
        );
        assert_eq!(
            index(&[1, 2, 3, 4], 69),
            None,
            "Failed on the third sample test"
        );
    }
}
