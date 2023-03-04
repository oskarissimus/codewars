#![allow(dead_code)]
fn feast(beast: &str, dish: &str) -> bool {
    beast[..1] == dish[..1] && beast.chars().last() == dish.chars().last()
}

// Rust test example:
#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}
