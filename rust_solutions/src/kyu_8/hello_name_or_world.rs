#![allow(dead_code)]
fn hello(name: &str) -> String {
    match name {
        "" => String::from("Hello, World!"),
        _ => format_name(name),
    }
}

fn format_name(name: &str) -> String {
    let first = &name[0..1].to_uppercase();
    let lowercase_name = name.to_lowercase();
    let rest = &lowercase_name[1..];
    let fromatted_name = format!("Hello, {}{}!", first, rest);
    fromatted_name
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
