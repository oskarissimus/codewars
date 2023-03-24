use itertools::Itertools;

fn order(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| (word, extract_digit(word)))
        .sorted_by_key(|(_, digit)| *digit)
        .map(|(word, _)| word)
        .join(" ")
}

fn extract_digit(word: &str) -> i32 {
    word.matches(char::is_numeric)
        .map(|c| i32::from_str_radix(c, 10).unwrap())
        .nth(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
