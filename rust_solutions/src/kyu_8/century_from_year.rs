#[allow(dead_code)]
fn century(year: u32) -> u32 {
    let d = year / 100;
    let remainder = year % 100;
    if remainder > 0 {
        return d + 1;
    }
    return d;
}

#[test]
fn basic_tests() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
}
