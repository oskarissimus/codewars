#![allow(dead_code)]
fn remove_char(s: &str) -> String {
    s.chars()
        .enumerate()
        .filter(|(i, _)| *i != 0)
        .map(|(_, v)| v)
        .collect::<String>()
        .chars()
        .rev()
        .enumerate()
        .filter(|(i, _)| *i != 0)
        .map(|(_, v)| v)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}
#[cfg(test)]
mod tests {
    use super::remove_char;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
}
