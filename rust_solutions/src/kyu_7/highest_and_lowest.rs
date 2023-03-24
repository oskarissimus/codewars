use itertools::Itertools;

fn high_and_low(numbers: &str) -> String {
    let result = numbers
        .split_whitespace()
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .minmax()
        .into_option()
        .unwrap();
    format!("{} {}", result.1, result.0)
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
