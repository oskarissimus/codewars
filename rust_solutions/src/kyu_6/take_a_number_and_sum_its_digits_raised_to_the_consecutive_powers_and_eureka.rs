#![allow(dead_code)]

use itertools::Itertools;
fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b).filter(|&x| check(x)).collect_vec()
}
fn check(n: u64) -> bool {
    n == n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u128)
        .enumerate()
        .map(|(index, value)| value.pow((index + 1).try_into().unwrap()))
        .sum::<u128>() as u64
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_dig_pow;

    fn dotest(a: u64, b: u64, expected: &[u64]) {
        let actual = sum_dig_pow(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1, 10, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        dotest(1, 100, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
        dotest(10, 89, &[89]);
        dotest(10, 100, &[89]);
        dotest(90, 100, &[]);
        dotest(89, 135, &[89, 135]);
    }
}
