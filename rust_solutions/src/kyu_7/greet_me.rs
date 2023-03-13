#![allow(dead_code)]
fn greet(name: &str) -> String {
    let first = name[..1].to_uppercase();
    let rest = name[1..].to_lowercase();
    format!("Hello {}{}!", first, rest)
}
#[cfg(test)]
mod tests {
    use super::greet;

    const ERR: &str = "\nYour result (left) did not match the expected output (right).";

    #[test]
    fn returns_expected() {
        assert_eq!(greet("riley"), "Hello Riley!", "{ERR}");
        assert_eq!(greet("JACK"), "Hello Jack!", "{ERR}");
    }
}
