#![allow(dead_code)]

use itertools::Itertools;
fn revrot(s: &str, c: usize) -> String {
    let n = s.len();
    if c <= 0 || c > n {
        return "".to_string();
    }
    let chunks = (c..=n)
        .step_by(c)
        .map(|end| s[end - c..end].to_string())
        .collect_vec();
    chunks
        .iter()
        .map(|chunk| {
            if sum_of_digits_is_divisible_by_2(chunk) {
                reverse(chunk)
            } else {
                rotate(chunk.to_string())
            }
        })
        .collect()
}
fn sum_of_digits_is_divisible_by_2(s: &String) -> bool {
    s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() % 2 == 0
}

fn rotate(mut s: String) -> String {
    let first = s.remove(0);
    s.push(first);
    s
}

fn reverse(s: &String) -> String {
    s.chars().rev().collect()
}

fn testing(s: &str, c: usize, exp: &str) -> () {
    assert_eq!(&revrot(s, c), exp)
}

#[test]
fn basics_revrot() {
    testing("1234", 0, "");
    testing("", 0, "");
    testing("1234", 5, "");
    let s = "733049910872815764";
    testing(s, 5, "330479108928157");
    let s = "73304991087281576455176044327690580265896";
    testing(s, 8, "1994033775182780067155464327690480265895");
}
