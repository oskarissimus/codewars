#![allow(dead_code)]
fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::between;

    fn dotest(a: i16, b: i16, expected: &[i16]) {
        let actual = between(a, b);
        assert!(
            actual == expected,
            "Test failed with a = {a}, b = {b}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn test_basic() {
        dotest(1, 4, &[1, 2, 3, 4]);
        dotest(-2, 2, &[-2, -1, 0, 1, 2]);
    }
}
