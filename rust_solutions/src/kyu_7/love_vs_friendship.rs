#![allow(dead_code)]
fn words_to_marks(s: &str) -> u32 {
    s.chars().fold(0, |sum, c| sum + c as u32 - 'a' as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
