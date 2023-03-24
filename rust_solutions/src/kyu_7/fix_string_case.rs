fn solve(s: &str) -> String {
    match s.matches(char::is_lowercase).count() {
        n if n >= s.len() / 2 => s.to_lowercase(),
        _ => s.to_ascii_uppercase(),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
