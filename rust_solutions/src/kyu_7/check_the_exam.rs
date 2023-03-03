#![allow(dead_code)]
fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let points = arr_a.iter().zip(arr_b).fold(0, |points, (correct, given)| {
        points
            + match *given {
                answer if &answer == correct => 4,
                "" => 0,
                _ => -1,
            }
    });
    match points {
        p if p > 0 => p,
        _ => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]), 0);
    }
}
