#![allow(dead_code)]

use std::{
    cmp::{max, min},
    ops::Add,
};
fn get_sum(a: i64, b: i64) -> i64 {
    let (min, max) = (min(a, b), max(a, b));
    let mean = (a + b) as f64 / 2 as f64;
    let elements = (max - min).abs().add(1) as f64;
    (mean * elements) as i64
}

// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
