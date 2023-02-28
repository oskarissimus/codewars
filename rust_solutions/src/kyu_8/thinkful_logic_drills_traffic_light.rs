#![allow(dead_code)]
fn update_light(current: &str) -> String {
    match current {
        "green" => String::from("yellow"),
        "yellow" => String::from("red"),
        _ => String::from("green"),
    }
}

#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}
