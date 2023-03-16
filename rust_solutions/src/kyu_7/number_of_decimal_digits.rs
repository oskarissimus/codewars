fn digits(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    let mut number = n;
    while number > 0 {
        number /= 10;
        d += 1;
    }
    d
}

#[test]
fn sample_test() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(5), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
