#![allow(dead_code)]

use itertools::Itertools;
use std::iter::zip;
fn thirt(n: i64) -> i64 {
    let remainders = [1, 10, 9, 12, 3, 4];
    let digits = n
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect_vec();
    let cycled_remainders = (0..digits.len())
        .map(|i| remainders[i % 6] as i64)
        .collect_vec();
    let s = zip(digits, cycled_remainders).map(|(d, r)| d * r).sum();
    match s {
        x if x == n => n,
        x => thirt(x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_thirt() {
        assert_eq!(thirt(8529), 79);
        assert_eq!(thirt(85299258), 31);
        assert_eq!(thirt(5634), 57);
    }
}
