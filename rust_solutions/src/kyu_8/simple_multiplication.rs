#![allow(dead_code)]
fn simple_multiplication(number: u8) -> u8 {
    match number % 2 {
        1 => number * 9,
        _ => number * 8,
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }
}
