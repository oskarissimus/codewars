fn hero(bullets: u16, dragons: u16) -> bool {
    match dragons {
        0 => true,
        _ => bullets / dragons >= 2,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
        assert_eq!(hero(1, 0), true);
    }
}
