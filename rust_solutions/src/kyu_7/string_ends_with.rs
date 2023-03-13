#![allow(dead_code)]
fn solution(word: &str, ending: &str) -> bool {
    if word.len() >= ending.len() {
        let n = word.len() - ending.len();
        word[n..] == *ending
    } else {
        false
    }
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}
