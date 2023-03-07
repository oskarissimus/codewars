#![allow(dead_code)]

use itertools::Itertools;
fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .map(|line| (1..=len).map(|col| col * line).collect_vec())
        .collect_vec()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    }
}
