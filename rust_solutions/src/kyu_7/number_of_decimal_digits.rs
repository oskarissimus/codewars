
fn digits(n: u64) -> usize {
    let mut number = n;
    let mut digits = 1;

    while number >= 10 {
        number /= 10;
        digits += 1;
    }

    digits
}

#[test]
fn sample_test() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(5), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
    assert_eq!(digits(999999999999999), 15);
}
