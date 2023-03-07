#![allow(dead_code, unused_mut)]

use itertools::Itertools;
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits
        .iter()
        .sorted_unstable()
        .dedup()
        .join("")
        .parse::<i32>()
        .unwrap()
}

#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
