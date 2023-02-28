#![allow(dead_code)]
fn get_age(age: &str) -> u32 {
    match age.chars().nth(0) {
        Some(c) => (c as u32) - ('0' as u32),
        _ => panic!("bad input"),
    }
}

#[test]
fn basic_tests() {
    assert_eq!(get_age("2 years old"), 2);
    assert_eq!(get_age("4 years old"), 4);
    assert_eq!(get_age("5 years old"), 5);
    assert_eq!(get_age("7 years old"), 7);
}
