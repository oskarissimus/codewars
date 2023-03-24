use itertools::Itertools;

fn min_sum(xs: &[u64]) -> u64 {
    let len = xs.len();
    let sorted = xs.iter().sorted_unstable();
    let first_half = sorted.clone().take(len / 2);
    let second_half = sorted.clone().skip(len / 2);
    first_half.zip(second_half.rev()).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5, 4, 2, 3]), 22);
        assert_eq!(min_sum(&[12, 6, 10, 26, 3, 24]), 342);
        assert_eq!(min_sum(&[9, 2, 8, 7, 5, 4, 0, 6]), 74);
    }
}
