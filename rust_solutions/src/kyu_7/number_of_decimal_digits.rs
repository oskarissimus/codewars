fn digits(n: u64) -> usize {
    (1..)
        .take_while(|i| 10u64.checked_pow(*i as u32).unwrap_or(0) <= n)
        .count()
        + 1
}

#[test]
fn sample_test() {
    // assert_eq!(digits(0), 1);
    assert_eq!(digits(5), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
    assert_eq!(digits(999999999999999), 15);
}
