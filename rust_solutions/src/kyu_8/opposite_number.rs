pub fn opposite(num: i32) -> i32 {
    -num
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-1), 1);
    }
}
