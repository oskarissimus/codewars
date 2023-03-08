#![allow(dead_code)]

use itertools::Itertools;
fn powers_of_two(n: u8) -> Vec<u128> {
    (0..=n).map(|x| 2_u128.pow(x.into())).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::powers_of_two;

    fn dotest(n: u8, expected: &[u128]) {
        let actual = powers_of_two(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(0, &[1]);
        dotest(1, &[1, 2]);
        dotest(4, &[1, 2, 4, 8, 16]);
    }
}
