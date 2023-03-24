use itertools::Itertools;

fn min_sum(xs: &[u64]) -> u64 {
    xs.iter()
        .permutations(xs.len())
        .map(|permutation| permutation.chunks(2).map(|chunk| chunk[0] * chunk[1]).sum())
        .min()
        .unwrap()
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
