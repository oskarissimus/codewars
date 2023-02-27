#![allow(dead_code)]
fn hello(name: &str) -> String {
    match name {
        "" => String::from("Hello, World!"),
        _ => format_name(name),
    }
}

fn format_name(name: &str) -> String {
    let first = name[..1].to_uppercase();
    let rest = name[1..].to_lowercase();
    format!("Hello, {}{}!", first, rest)
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
